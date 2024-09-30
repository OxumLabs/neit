use super::{
    maths::evaluate_expression,
    tokens::{func::process_func, print::process_print, var::process_var},
    types::{Args, Tokens},
};

#[allow(unused, irrefutable_let_patterns)]
pub fn gentoken(code: Vec<&str>) -> Result<Vec<Tokens>, String> {
    let mut index = 0;
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut in_function = false;
    let mut function_body = Vec::new();
    let mut brace_depth = 0;
    let mut p_label = 0;
    let mut fp_label = 364;

    for mut ln in code {
        if let Some(pos) = ln.find('#') {
            ln = &ln[..pos].trim(); // Remove comments
        }
        index += 1;
        ln = ln.trim(); // Trim whitespace from the line

        if ln.is_empty() {
            continue; // Skip empty lines
        }

        // Handle function definitions
        if (ln.trim().starts_with("pub fn") || ln.trim().starts_with("fn "))
            && ln.trim().ends_with("{")
        {
            if in_function {
                return Err(format!(
                    "Error at line {}: Unexpected function definition while inside another function.\n\
                    Hint: Close the current function before starting a new one.\n\
                    Code:\n   => {}",
                    index, ln
                ));
            }

            in_function = true;
            function_body.push(ln);

            brace_depth += ln.matches('{').count();
            brace_depth -= ln.matches('}').count();

            // Check if brace depth is balanced
            if brace_depth == 0 {
                in_function = false;
                function_body.clear();
            }
        } else if in_function {
            function_body.push(ln);

            brace_depth += ln.matches('{').count();
            brace_depth -= ln.matches('}').count();

            // Process the function body once braces are balanced
            if brace_depth == 0 {
                let full_function_code = function_body.join("\n");
                match process_func(&full_function_code, index, &mut fp_label) {
                    Ok(func) => {
                        if tokens
                            .iter()
                            .any(|tkn| matches!(tkn, Tokens::Func(f) if f.name == func.name))
                        {
                            return Err(format!(
                                "Error at line {}: Function '{}' is already declared.\n\
                                Hint: Ensure each function has a unique name.\n\
                                Code:\n   => {}",
                                index, func.name, full_function_code
                            ));
                        }
                        tokens.push(Tokens::Func(func));
                        in_function = false;
                        function_body.clear();
                    }
                    Err(e) => return Err(e),
                }
            }
        } else if (ln.trim().starts_with("may") && !ln.trim().starts_with("may whole"))
            && ln.contains('=')
        {
            let vr = process_var(ln.trim(), &tokens, false); // Trim the line before processing
            match vr {
                Ok(vr) => tokens.push(Tokens::Var(vr.0, vr.1, false)),
                Err(e) => return Err(e),
            }
        } else if ln.trim().starts_with("must ") {
            let vr = process_var(ln.trim(), &tokens, true); // Trim the line before processing
            match vr {
                Ok(vr) => tokens.push(Tokens::Var(vr.0, vr.1, true)),
                Err(e) => return Err(e),
            }
        } else if (ln.trim().starts_with("fn") || ln.trim().starts_with("pub fn"))
            && ln.trim().ends_with("{}")
        {
            match process_func(ln.trim(), index, &mut fp_label) {
                // Trim before processing
                Ok(f) => tokens.push(Tokens::Func(f)),
                Err(e) => return Err(e),
            }
        } else if ln.trim().starts_with("print(") && ln.trim().ends_with(")") {
            let txt = ln[6..ln.len() - 1].trim(); // Extract print arguments
            let ptxt = process_print(&mut p_label, txt, &tokens);
            tokens.push(ptxt);
        } else if ln.trim().starts_with("println(") && ln.trim().ends_with(")") {
            let mut txt: String = ln[8..ln.len() - 1].trim().to_string(); // Extract println arguments
            txt.push_str(r#"\n"#);
            let ptxt = process_print(&mut p_label, &txt, &tokens);
            tokens.push(ptxt);
        } else {
            let args: Vec<&str> = ln.trim().split('(').collect(); // Split on parentheses
            let mut found_function = false;

            if args.len() == 2 {
                let (nm, args_str) = (
                    args.first().unwrap(),
                    args.get(1).unwrap().trim_end_matches(')'),
                );

                let provided_args: Vec<&str> = args_str
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .collect();

                if let Some(Tokens::Func(f)) = tokens
                    .iter()
                    .find(|tkn| matches!(tkn, Tokens::Func(f) if f.name == *nm.trim()))
                {
                    let expected_args: Vec<Args> = f.args.clone();

                    // Validate the number of provided arguments
                    if provided_args.len() != expected_args.len() {
                        return Err(format!(
                            "Error at line {}: Function '{}' called with incorrect number of arguments.\n\
                            Hint: Expected {} arguments but got {}.\n\
                            Code:\n   => {}",
                            index, nm.trim(), expected_args.len(), provided_args.len(), ln
                        ));
                    }

                    // Validate the types of provided arguments
                    for (provided, expected) in provided_args.iter().zip(expected_args.iter()) {
                        let provided_type = match determine_type(provided) {
                            Ok(t) => t,
                            Err(e) => {
                                return Err(format!(
                                    "Error at line {}: Argument '{}' could not be parsed. {}\n\
                                    Hint: Ensure arguments are of correct type.\n\
                                    Code:\n   => {}",
                                    index, provided, e, ln
                                ));
                            }
                        };

                        let expected_type = match expected {
                            Args::Str(_) => "string",
                            Args::Int(_) => "int",
                            Args::Float(_) => "float",
                            _ => "unknown",
                        };

                        if provided_type != expected_type {
                            return Err(format!(
                                "Error at line {}: Argument type mismatch in function call '{}'.\n\
                                Hint: Expected argument type '{}' but got '{}'.\n\
                                Code:\n   => {}",
                                index,
                                nm.trim(),
                                expected_type,
                                provided_type,
                                ln
                            ));
                        }
                    }

                    tokens.push(Tokens::FnCall(nm.trim().to_string()));
                    found_function = true;
                }
            }

            // Handle variable assignment
            if !found_function {
                let mut vfnd = false;
                for v in &tokens.clone() {
                    match v {
                        Tokens::Var(vr, n, c) => {
                            let ln = ln.trim();
                            if let Some(pos) = ln.find(&n.trim()) {
                                let v = ln[pos + n.len()..].trim(); // Trim after the variable name
                                if v.starts_with("=") {
                                    let pts: Vec<&str> = v.split('=').collect();
                                    if pts.len() == 2 {
                                        let val = pts.get(1).unwrap().trim(); // Trim the assigned value
                                        if val.contains("+")
                                            || val.contains("-")
                                            || val.contains("*")
                                            || val.contains("/")
                                            || val.contains("%")
                                        {
                                            match evaluate_expression(&val, &tokens) {
                                                Ok(v) => {
                                                    tokens.push(Tokens::Revar(
                                                        n.to_string(),
                                                        v.to_string(),
                                                    ));
                                                    vfnd = true;
                                                }
                                                Err(e) => return Err(e),
                                            }
                                        } else {
                                            // Handle direct assignment (no expression)
                                            tokens.push(Tokens::Revar(
                                                n.to_string(),
                                                val.to_string(),
                                            ));
                                            vfnd = true;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if !vfnd {
                    return Err(format!(
                        "Error at line {}: Invalid function call or unmatched function name.\n\
                    Hint: Ensure function calls match declared function names.\n\
                    Code:\n   => {}",
                        index, ln
                    ));
                }
            }
        }
    }

    // Final check for unbalanced braces
    if in_function {
        if brace_depth > 0 {
            return Err(format!(
                "Error: Unbalanced braces. Function starting at line {} is not closed.\n\
                Hint: Check if all opening braces have matching closing braces.\n\
                Code:\n   => {}",
                index,
                function_body.join("\n")
            ));
        }
        let full_function_code = function_body.join("\n");
        match process_func(&full_function_code, index, &mut fp_label) {
            Ok(func) => {
                tokens.push(Tokens::Func(func));
                Ok(tokens)
            }
            Err(e) => Err(e),
        }
    } else {
        Ok(tokens)
    }
}

// Helper function to determine the type of an argument
fn determine_type(arg: &str) -> Result<&'static str, String> {
    let trimmed = arg.trim(); // Trim the argument

    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        Ok("string")
    } else if trimmed.starts_with('\'') && trimmed.ends_with('\'') {
        Ok("string")
    } else if trimmed.parse::<i32>().is_ok() {
        Ok("int")
    } else if trimmed.parse::<f64>().is_ok() {
        Ok("float")
    } else {
        Err(format!(
            "Error: Argument '{}' does not match expected types (string, int, or float).\n\
            Hint: Ensure the argument type is correct.\n\
            Code:\n   => {}",
            arg, arg
        ))
    }
}
