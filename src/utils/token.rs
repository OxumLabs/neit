use super::{
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
            ln = &ln[..pos].trim();
        }
        index += 1;
        ln = ln.trim();

        if ln.is_empty() {
            continue;
        }

        if (ln.trim().starts_with("pub fn") || ln.trim().starts_with("fn ")) && ln.ends_with("{") {
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

            if brace_depth == 0 {
                in_function = false;
                function_body.clear();
            }
        } else if in_function {
            function_body.push(ln);

            brace_depth += ln.matches('{').count();
            brace_depth -= ln.matches('}').count();

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
        } else if (ln.starts_with("may ") && !ln.starts_with("may whole ")) && ln.contains("=") {
            let vr = process_var(ln, &tokens);
            match vr {
                Ok(vr) => tokens.push(Tokens::Var(vr.0, vr.1, false)),
                Err(e) => return Err(e),
            }
        } else if (ln.starts_with("fn ") || ln.starts_with("pub fn ")) && ln.ends_with("{}") {
            match process_func(ln, index, &mut fp_label) {
                Ok(f) => tokens.push(Tokens::Func(f)),
                Err(e) => return Err(e),
            }
        } else if ln.starts_with("print(") && ln.ends_with(")") {
            let txt = ln[6..].trim_end_matches(")");
            let ptxt = process_print(&mut p_label, txt, &tokens);
            tokens.push(ptxt);
        } else if ln.starts_with("println(") && ln.ends_with(")") {
            let mut txt: String = ln[8..].trim_end_matches("\")").to_string();
            //txt.trim_end_matches("\"");
            txt.push_str(r#"\n""#);
            let ptxt = process_print(&mut p_label, &txt, &tokens);
            tokens.push(ptxt);
        } else {
            let args: Vec<&str> = ln.trim().split('(').collect();
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
                    .find(|tkn| matches!(tkn, Tokens::Func(f) if f.name == *nm))
                {
                    let expected_args: Vec<Args> = f.args.clone();

                    if provided_args.len() != expected_args.len() {
                        return Err(format!(
                            "Error at line {}: Function '{}' called with incorrect number of arguments.\n\
                            Hint: Expected {} arguments but got {}.\n\
                            Code:\n   => {}",
                            index, nm, expected_args.len(), provided_args.len(), ln
                        ));
                    }

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
                                index, nm, expected_type, provided_type, ln
                            ));
                        }
                    }

                    tokens.push(Tokens::FnCall(nm.to_string()));
                    found_function = true;
                }
            }

            if !found_function {
                return Err(format!(
                    "Error at line {}: Invalid function call or unmatched function name.\n\
                    Hint: Ensure function calls match declared function names.\n\
                    Code:\n   => {}",
                    index, ln
                ));
            }
        }
    }

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

fn determine_type(arg: &str) -> Result<&'static str, String> {
    let trimmed = arg.trim();

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
