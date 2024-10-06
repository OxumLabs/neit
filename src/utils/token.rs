use crate::utils::case::process_case;

use super::{
    tokens::{func::process_func, input::process_input, print::process_print, var::process_var},
    types::{Args, Tokens, Vars},
};
use colored::*; // Import the colored crate

#[allow(unused, irrefutable_let_patterns)]
pub fn gentoken(code: Vec<&str>, casetkns: Vec<Tokens>, fc: bool) -> Result<Vec<Tokens>, String> {
    println!(
        "from case? {} | tokens? {}",
        fc,
        casetkns
            .iter()
            .map(|i| format!("{}", i)) // Format each token
            .collect::<Vec<_>>() // Collect formatted tokens into a Vec
            .join(", ") // Join them with a comma separator
    );

    let mut index: i64 = 0;
    let mut tokens: Vec<Tokens> = casetkns;
    let mut ct: Vec<Tokens> = Vec::new();
    let mut in_function = false;
    let mut function_body = Vec::new();
    let mut brace_depth = 0;
    let mut p_label = 0;
    let mut fp_label = 364;

    for mut ln in code.clone() {
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
                    "{} Oh no, rookie move! Found another function at line {} while you're still inside one.\n\
                     → First finish what you started before moving on!\n\
                     Code:\n   => {}\n\
                     Seriously, let’s close that function off before we get ahead of ourselves, okay?",
                    "✘".red(), index, ln
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
                match process_func(
                    &full_function_code,
                    index.try_into().unwrap(),
                    &mut fp_label,
                ) {
                    Ok(func) => {
                        if tokens
                            .iter()
                            .any(|tkn| matches!(tkn, Tokens::Func(f) if f.name == func.name))
                        {
                            return Err(format!(
                                "{} Yikes! The function '{}' is already declared at line {}. Rookie mistake, am I right?\n\
                                 → Ever heard of unique names? Let's give that function a fresh one!\n\
                                 Code:\n   => {}\n\
                                 Keep it unique, champ!",
                                "✘".red(), func.name, index, full_function_code
                            ));
                        }
                        if !fc {
                            tokens.push(Tokens::Func(func));
                        } else {
                            ct.push(Tokens::Func(func));
                        }
                        in_function = false;
                        function_body.clear();
                    }
                    Err(e) => return Err(e),
                }
            }
        } else if ln.starts_with("case ") && ln.ends_with("{") {
            let cname = ln[5..].trim_end_matches("}");
            // Initialize a vector to collect case lines
            let mut case_lines = Vec::new(); // Start with the case declaration
            let mut brace_depth = 1; // We've already seen one opening brace
            let mut inner_index = index + 1; // Start from the next line after the case declaration

            // Collect lines until the brace depth returns to zero
            while inner_index < code.len().try_into().unwrap() {
                let next_line = code[inner_index as usize].trim();

                // Check for empty lines
                if next_line.is_empty() {
                    inner_index += 1;
                    continue;
                }

                // Count braces
                brace_depth += next_line.matches('{').count();
                brace_depth -= next_line.matches('}').count();

                // Collect the line
                case_lines.push(next_line.to_string());

                // If brace depth is zero, we found the end of the case block
                if brace_depth == 0 {
                    break;
                }

                inner_index += 1; // Move to the next line
            }

            // Now we can call process_case with the collected case code
            let case_result = process_case(ln, case_lines, &mut index, true);
            (if let Err(e) = case_result {
                return Err(e);
            });
            tokens.push(Tokens::IFun(cname.to_string(), case_result.unwrap()));

            // Update the main index to continue processing after the case block
            index = inner_index; // Update the index to the position after the last collected line
        } else if (ln.trim().starts_with("may") && !ln.trim().starts_with("may whole"))
            && ln.contains('=')
        {
            let vr = process_var(ln.trim(), &tokens, true);
            match vr {
                Ok(vr) => {
                    if fc {
                        ct.push(Tokens::Var(vr.0, vr.1, true)); // Add to ct if fc is true
                    } else {
                        tokens.push(Tokens::Var(vr.0, vr.1, true)); // Add to tokens if fc is false
                    }
                }

                Err(e) => return Err(e),
            }
        } else if ln.trim().starts_with("must ") {
            let vr = process_var(ln.trim(), &tokens, false);
            match vr {
                Ok(vr) => {
                    if fc {
                        ct.push(Tokens::Var(vr.0, vr.1, false)); // Add to ct if fc is true
                    } else {
                        tokens.push(Tokens::Var(vr.0, vr.1, false)); // Add to tokens if fc is false
                    }
                }
                Err(e) => return Err(e),
            }
        } else if (ln.trim().starts_with("fn") || ln.trim().starts_with("pub fn"))
            && ln.trim().ends_with("{}")
        {
            match process_func(ln.trim(), index.try_into().unwrap(), &mut fp_label) {
                Ok(f) => {
                    if !fc {
                        tokens.push(Tokens::Func(f))
                    } else {
                        ct.push(Tokens::Func(f));
                    }
                }
                Err(e) => return Err(e),
            }
        } else if ln.trim().starts_with("print(") && ln.trim().ends_with(")") {
            let mut txt: String = ln[6..ln.len() - 1].trim().to_string(); // Extract println arguments
            let ptxt = process_print(&mut p_label, &txt, &tokens);
            if !fc {
                tokens.push(ptxt);
            } else {
                ct.push(ptxt);
            }
        } else if ln.starts_with("takein(") {
            let tkn = process_input(&ln, &tokens);
            match tkn {
                Ok(tkn) => {
                    if !fc {
                        tokens.push(tkn);
                    } else {
                        ct.push(tkn);
                    }
                }
                Err(e) => return Err(e),
            }
        } else if ln.trim().starts_with("println(") && ln.trim().ends_with(")") {
            let mut txt: String = ln[9..ln.len() - 2].to_string(); // Extract println arguments
            let txt = format!(r#""{}\n""#, txt);
            let ptxt = process_print(&mut p_label, &txt, &tokens);
            if !fc {
                tokens.push(ptxt);
            } else {
                ct.push(ptxt);
            }
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
                            "{} Oops! Looks like you called the function '{}' at line {} with the wrong number of arguments.\n\
                             → Expected {}, but you gave me {}. Rookie mistake, right?\n\
                             Code:\n   => {}\n\
                             Let’s fix that up, shall we?",
                            "✘".red(), nm.trim(), index, expected_args.len(), provided_args.len(), ln
                        ));
                    }

                    for (provided, expected) in provided_args.iter().zip(expected_args.iter()) {
                        let provided_type =
                            match determine_type(provided, &tokens) {
                                Ok(t) => t,
                                Err(_) => {
                                    return Err(format!(
                                    "{} Are you kidding me? I can't even parse '{}' at line {}.\n\
                                 → Double check that argument—I'm begging you!\n\
                                 Code:\n   => {}",
                                    "✘".red(), provided, index, ln
                                ))
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
                                "{} Oh no! The argument '{}' doesn't match the expected type '{}'. Line {}:\n\
                                 Code:\n   => {}\n\
                                 Let's get our types sorted out, shall we?",
                                "✘".red(), provided, expected_type, index, ln
                            ));
                        }
                    }

                    if !fc {
                        tokens.push(Tokens::FnCall(f.clone().name, provided_args));
                    // Push to tokens
                    } else {
                        ct.push(Tokens::FnCall(f.clone().name, provided_args)); // Push to ct
                    }

                    found_function = true; // Indicate that a function call was found
                }
            }

            if !found_function {
                if ln.chars().all(|c| c == '}') {
                    continue;
                }
                return Err(format!(
                    "{} So, about that line {}... I couldn't quite figure out what you meant.\n\
                     → Make sure to double-check your syntax.\n\
                     Code:\n   => {}\n\
                     It's a bit of a head-scratcher, I know.",
                    "✘".red(),
                    index,
                    ln
                ));
            }
        }
    }

    if fc {
        return Ok(ct);
    }
    Ok(tokens) // Return the generated tokens
}

/// A function to process case statements separately.

pub fn determine_type(arg: &str, tokens: &Vec<Tokens>) -> Result<&'static str, String> {
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
