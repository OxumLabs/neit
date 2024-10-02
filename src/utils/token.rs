use super::{
    maths::evaluate_expression,
    tokens::{func::process_func, print::process_print, var::process_var},
    types::{Args, Tokens, Vars},
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
            ln = ln[..pos].trim(); // Remove comments
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
            let vr = process_var(ln.trim(), &tokens, true);
            match vr {
                Ok(vr) => tokens.push(Tokens::Var(vr.0, vr.1, false)),
                Err(e) => return Err(e),
            }
        } else if ln.trim().starts_with("must ") {
            let vr = process_var(ln.trim(), &tokens, false);
            match vr {
                Ok(vr) => tokens.push(Tokens::Var(vr.0, vr.1, false)),
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
            let mut txt: String = ln[6..ln.len() - 1].trim().to_string(); // Extract println arguments
                                                                          //let txt = format!(r#""\n{}""#, txt);
            let ptxt = process_print(&mut p_label, &txt, &tokens);
            tokens.push(ptxt);
        } else if ln.trim().starts_with("println(") && ln.trim().ends_with(")") {
            let mut txt: String = ln[9..ln.len() - 2].trim().to_string(); // Extract println arguments
            let txt = format!(r#""\n{}""#, txt);
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

                let provided_args: Vec<String> = args_str
                    .split(',')
                    .map(|s| s.trim().to_string()) // Convert &str to String after trimming
                    .filter(|s| !s.is_empty()) // Filter out empty strings
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
                        let provided_type = match determine_type(provided, &tokens) {
                            Ok(t) => t,
                            Err(e) => {
                                return Err(format!(
                                    "Error at line {}: Argument '{}' could not be parsed. {}\n\
                                    Hint: Ensure arguments are of correct type (string, int, float).\n\
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

                        // Allow int to float conversion
                        if provided_type != expected_type
                            && !(provided_type == "float" && expected_type == "int")
                            && !(provided_type == "int" && expected_type == "float")
                        {
                            return Err(format!(
                                "Error at line {}: Argument type mismatch in function call '{}'.\n\
                                Expected argument type '{}' but got '{}'.\n\
                                Code:\n   => {}\n\
                                Hint: Check if the argument is convertible or matches expected type.",
                                index,
                                nm.trim(),
                                expected_type,
                                provided_type,
                                ln
                            ));
                        }
                    }

                    tokens.push(Tokens::FnCall(nm.trim().to_string(), provided_args));
                    found_function = true;
                }
            }

            // Handle variable assignment
            if !found_function {
                let mut vfnd = false;
                for v in &tokens.clone() {
                    if let Tokens::Var(vr, n, c) = v {
                        let ln = ln.trim();
                        if let Some(pos) = ln.find(n.trim()) {
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
                                        match evaluate_expression(val, &tokens) {
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
                                        tokens.push(Tokens::Revar(n.to_string(), val.to_string()));
                                        vfnd = true;
                                    }
                                }
                            }
                        }
                    }
                }

                if !vfnd {
                    // Handle function call
                    let args: Vec<&str> = ln.trim().split('(').collect(); // Split on parentheses
                    if args.len() == 2 {
                        let (nm, args_str) = (
                            args.first().unwrap(),
                            args.get(1).unwrap().trim_end_matches(')'),
                        );

                        let provided_args: Vec<String> = args_str
                            .split(',')
                            .map(|s| s.trim().to_string())
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
                            for (provided, expected) in
                                provided_args.iter().zip(expected_args.iter())
                            {
                                let provided_type = match determine_type(provided, &tokens) {
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

                                // Allow int to float conversion
                                if provided_type != expected_type
                                    && !(provided_type == "float" && expected_type == "int")
                                    && !(provided_type == "int" && expected_type == "float")
                                {
                                    return Err(format!(
                                        "Error at line {}: Argument type mismatch in function call '{}'.\n\
                                        Expected argument type '{}' but got '{}'.\n\
                                        Code:\n   => {}\n\
                                        Hint: Check if the argument is convertible or matches expected type.",
                                        index,
                                        nm.trim(),
                                        expected_type,
                                        provided_type,
                                        ln
                                    ));
                                }
                            }

                            tokens.push(Tokens::FnCall(nm.trim().to_string(), provided_args));
                        }
                    }
                }
            }
        }
    }

    Ok(tokens)
}

// Updated determine_type function
fn determine_type(arg: &str, tokens: &Vec<Tokens>) -> Result<&'static str, String> {
    let trimmed = arg.trim(); // Trim the argument
    for t in tokens {
        match t {
            Tokens::Var(v, n, _) => match v {
                Vars::STR(_) => {
                    if n == arg {
                        return Ok("string");
                    } else {
                        continue;
                    };
                }
                Vars::INT(_) => {
                    if n == arg {
                        return Ok("int");
                    } else {
                        continue;
                    }
                }
                Vars::F(_) => {
                    if n == arg {
                        return Ok("float");
                    } else {
                        continue;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    // Determine type based on string format
    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        Ok("string")
    } else if trimmed.starts_with('\'') && trimmed.ends_with('\'') {
        Ok("string")
    } else if trimmed.parse::<i128>().is_ok() {
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
