use super::{tokens::func::process_func, types::Tokens};

#[allow(unused, irrefutable_let_patterns)]
pub fn gentoken(code: Vec<&str>) -> Result<Vec<Tokens>, String> {
    let mut index = 0;
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut in_function = false;
    let mut function_body = Vec::new();
    let mut brace_depth = 0;

    for ln in code.clone() {
        index += 1;

        if ln.is_empty() {
            continue;
        }

        if (ln.trim().starts_with("pub fn") || ln.trim().starts_with("fn ")) && !ln.ends_with("}") {
            if in_function {
                return Err(format!(
                    "Error at line {}: Unexpected function definition while inside another function.\nHint: Close the current function before starting a new one.\nCode:\n   => {}",
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
                match process_func(&full_function_code, index) {
                    Ok(func) => {
                        if tokens
                            .iter()
                            .any(|tkn| matches!(tkn, Tokens::Func(f) if f.name == func.name))
                        {
                            return Err(format!(
                                "Error at line {}: Function '{}' is already declared.\nHint: Ensure each function has a unique name.\nCode:\n   => {}",
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
        } else if (ln.starts_with("fn ") || ln.starts_with("pub fn ")) && ln.ends_with("{}") {
            match process_func(ln, index) {
                Ok(f) => tokens.push(Tokens::Func(f)),
                Err(e) => return Err(e),
            }
        } else {
            let mut fe = false;
            return Err(format!(
                "Error at line {}: Invalid code format.\nHint: Ensure function declarations and other statements are properly formatted.\nCode:\n   => {}",
                index, ln
            ));
        }
    }

    if in_function {
        if brace_depth > 0 {
            return Err(format!(
                "Error: Unbalanced braces. Function starting at line {} is not closed.\nHint: Check if all opening braces have matching closing braces.\nCode:\n   => {}",
                index, function_body.join("\n")
            ));
        }
        let full_function_code = function_body.join("\n");
        match process_func(&full_function_code, index) {
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
