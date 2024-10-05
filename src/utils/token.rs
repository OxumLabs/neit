use std::process::exit;

use super::{
    tokens::{func::process_func, input::process_input, print::process_print, var::process_var},
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
                    "✘ Oh no, rookie move! Found another function at line {} while you're still inside one.\n\
                     → First finish what you started before moving on!\n\
                     Code:\n   => {}\n\
                     Seriously, let’s close that function off before we get ahead of ourselves, okay?",
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
                                "✘ Yikes! The function '{}' is already declared at line {}. Rookie mistake, am I right?\n\
                                 → Ever heard of unique names? Let's give that function a fresh one!\n\
                                 Code:\n   => {}\n\
                                 Keep it unique, champ!",
                                func.name, index, full_function_code
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
                Ok(vr) => tokens.push(Tokens::Var(vr.0, vr.1, true)),
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
                Ok(f) => tokens.push(Tokens::Func(f)),
                Err(e) => return Err(e),
            }
        } else if ln.trim().starts_with("print(") && ln.trim().ends_with(")") {
            let mut txt: String = ln[6..ln.len() - 1].trim().to_string(); // Extract println arguments
            let ptxt = process_print(&mut p_label, &txt, &tokens);
            tokens.push(ptxt);
        } else if ln.starts_with("takein(") {
            let tkn = process_input(&ln, &tokens);
            match tkn {
                Ok(tkn) => {
                    tokens.push(tkn);
                }
                Err(e) => return Err(e),
            }
        } else if ln.trim().starts_with("println(") && ln.trim().ends_with(")") {
            let mut txt: String = ln[9..ln.len() - 2].to_string(); // Extract println arguments
            let txt = format!(r#""{}\n""#, txt);
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

                    if provided_args.len() != expected_args.len() {
                        return Err(format!(
                            "✘ Oops! Looks like you called the function '{}' at line {} with the wrong number of arguments.\n\
                             → Expected {}, but you gave me {}. Rookie mistake, right?\n\
                             Code:\n   => {}\n\
                             Let’s fix that up, shall we?",
                            nm.trim(), index, expected_args.len(), provided_args.len(), ln
                        ));
                    }

                    for (provided, expected) in provided_args.iter().zip(expected_args.iter()) {
                        let provided_type = match determine_type(provided, &tokens) {
                            Ok(t) => t,
                            Err(_) => {
                                return Err(format!(
                                    "✘ Are you kidding me? I can't even parse '{}' at line {}.\n\
                                 → Double check that argument—I'm begging you!\n\
                                 Code:\n   => {}",
                                    provided, index, ln
                                ))
                            }
                        };

                        let expected_type = match expected {
                            Args::Str(_) => "string",
                            Args::Int(_) => "int",
                            Args::Float(_) => "float",
                            _ => "unknown",
                        };

                        if provided_type != expected_type
                            && !(provided_type == "float" && expected_type == "int")
                            && !(provided_type == "int" && expected_type == "float")
                        {
                            return Err(format!(
                                "✘ Uh-oh, mismatch alert at line {}! You called '{}' with the wrong argument types.\n\
                                 → Expected '{}', but you gave me '{}'. Come on, you can do better!\n\
                                 Code:\n   => {}\n\
                                 Let's try that again, shall we?",
                                index, nm.trim(), expected_type, provided_type, ln
                            ));
                        }
                    }

                    tokens.push(Tokens::FnCall(nm.trim().to_string(), provided_args));
                    found_function = true;
                }
            }

            if !found_function {
                if ln.ends_with(";") {
                    eprintln!("Error at line '{}': '{}'. \nAha! There it is, the notorious semicolon! \nDid you think you could just slip it in here and nobody would notice? \nSurprise! We see you, and it’s time to face the music.\nThis language is semicolon-free, so send it back where it belongs!\nLet’s keep our code elegant and simple!", index, ln);
                    exit(1);
                } else {
                    return Err(format!(
                        "✘ What the...? I found something strange at line {}.\n\
                     → '{}'? Really? You sure about that?\n\
                     Let’s rethink that one, yeah?",
                        index, ln
                    ));
                }
            }
        }
    }
    Ok(tokens)
}

fn determine_type(arg: &str, tokens: &Vec<Tokens>) -> Result<&'static str, String> {
    let trimmed = arg.trim(); // Trim the argument
    for t in tokens {
        match t {
            Tokens::Var(v, n, _) => match v {
                Vars::STR(_) => {
                    if n == arg {
                        return Ok("string");
                    }
                }
                Vars::INT(_) => {
                    if n == arg {
                        return Ok("int");
                    }
                }
                Vars::F(_) => {
                    if n == arg {
                        return Ok("float");
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        Ok("string")
    } else if trimmed.starts_with('\'') && trimmed.ends_with('\'') {
        Ok("string")
    } else if trimmed.parse::<i128>().is_ok() {
        Ok("int")
    } else if trimmed.parse::<f64>().is_ok() {
        Ok("float")
    } else {
        return Err(format!(
            "✘ Oh wow... '{}' isn't even a valid type. Come on now, you should know better.\n\
             → Let’s stick to int, float, or string, okay?\n\
             Code:\n   => {}",
            arg, arg
        ));
    }
}
