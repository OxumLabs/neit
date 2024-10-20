use super::{
    maths::evaluate_expression,
    tokens::{input::process_input, print::process_print, var::process_var},
    types::{fvars, Args, Tokens},
};

#[allow(unused)]
pub fn parse_single_line(
    line: &str,
    line_number: usize,
    p_label: &mut i32,
    lv: &mut Vec<fvars>,
    vars: &mut Vec<Tokens>,
    f_args: &[Args],
) -> Result<Tokens, String> {
    //println!("first cal ln in ftokesn : {}", line);

    let line = line.trim();
    if line.is_empty() {
        return Err("|_EMP_|".to_string());
    }

    // Return an error if a function declaration is encountered.
    if line.starts_with("fn ") || line.starts_with("pub fn") {
        return Err(format!(
            "✘ Whoa there! Looks like you tried to declare a function at line {}—and that's a big no-no!\n\
            → Reason: Functions are a bit picky and don’t like to be placed here (inside another function!!! Can you imagine?).\n\
            →→ Hint: Try moving them to a more suitable spot!\n\
            Code:\n   => {}",
            line_number, line
        ));
    }

    // Process print statements
    if line.starts_with("print(") && line.ends_with(")") {
        let txt = line[6..].trim_end_matches(")");
        *p_label += 365;
        let print_token = process_print(p_label, txt, vars);
        return Ok(print_token);
    } else if line.starts_with("println(") && line.ends_with(")") {
        let mut txt = line[8..].trim_end_matches("\")").to_string();
        txt.push_str(r#"\n""#);
        let txt = txt.as_str();
        *p_label += 365;
        let print_token = process_print(p_label, txt, vars);
        return Ok(print_token);
    } else if line.starts_with("takein(") {
        let tkn = process_input(&line, &vars);
        match tkn {
            Ok(tkn) => {
                //vars.push(tkn.clone());
                return Ok(tkn);
            }
            Err(e) => return Err(e),
        }
    } else if line.starts_with("may ") && line.contains("=") {
        let vr = process_var(line, vars, true);
        match vr {
            Ok(vr) => {
                lv.push(fvars {
                    v: vr.clone().0,
                    n: vr.clone().1,
                });
                vars.push(Tokens::Var(vr.clone().0, vr.clone().1, true));
                return Ok(Tokens::Var(vr.0, vr.1, true));
            }
            Err(e) => return Err(e),
        }
    } else if line.starts_with("must ") {
        let vr = process_var(line, vars, false);
        match vr {
            Ok(vr) => {
                lv.push(fvars {
                    v: vr.clone().0,
                    n: vr.clone().1,
                });
                vars.push(Tokens::Var(vr.clone().0, vr.clone().1, false));
                return Ok(Tokens::Var(vr.0, vr.1, false));
            }
            Err(e) => return Err(e),
        }
    }

    let mut vfnd = false;
    for v in vars.iter() {
        if let Tokens::Var(vr, n, c) = v {
            let ln = line.trim();
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
                            match evaluate_expression(val, vars) {
                                Ok(v) => {
                                    vfnd = true;
                                    return Ok(Tokens::Revar(n.to_string(), v.to_string()));
                                }
                                Err(e) => return Err(e),
                            }
                        } else {
                            // Handle direct assignment (no expression)
                            vfnd = true;
                            return Ok(Tokens::Revar(n.to_string(), val.to_string()));
                        }
                    }
                }
            }
        }
    }

    // Handle function calls (not declarations)
    let args: Vec<&str> = line.trim().split('(').collect();
    if args.len() == 2 {
        let (nm, args_str) = (
            args.first().unwrap(),
            args.get(1).unwrap().trim_end_matches(')'),
        );

        // Remove any empty arguments
        let provided_args: Vec<String> = args_str
            .split(',')
            .map(|s| s.trim().to_string()) // Convert &str to String after trimming
            .filter(|s| !s.is_empty()) // Filter out empty strings
            .collect();

        let expected_args: Vec<Args> = f_args.to_vec();

        if provided_args.len() != expected_args.len() {
            return Err(format!(
                "✘ Error: Incorrect Number of Arguments\n\
                At line {}: Function '{}' called with the wrong number of arguments.\n\
                ➔ Expected: {}, Provided: {}.\n\
                ➔ Suggested Action: Ensure your call matches the function's required arguments.\n\
                ➔ Hint: Check the function definition for the expected number of arguments.\n\n\
                Code:\n   => {}\n\
                Let's keep every function happy with the correct number of arguments!",
                line_number,
                nm,
                expected_args.len(),
                provided_args.len(),
                line
            ));
        }

        for (provided, expected) in provided_args.iter().zip(expected_args.iter()) {
            let provided_type = match determine_type(provided) {
                Ok(t) => t,
                Err(e) => {
                    return Err(format!(
                        "✘ Error: Invalid Argument at Line {}\n\
                        At line {}, I couldn't process the argument '{}'.\n\
                        ➔ Reason: {}\n\
                        ➔ Suggested Action: Ensure your arguments match the expected types for this function.\n\
                        ➔ Hint: Double-check the type of data you're passing—be it integer, string, or float.\n\n\
                        Code:\n   => {}",
                        line_number,
                        line_number,
                        provided,
                        e,
                        line
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
                    "✘ Error: Argument Type Mismatch at Line {}\n\
                    At line {}, there's a type mismatch in the function call '{}'.\n\
                    ➔ Expected: '{}', but received: '{}'.\n\
                    ➔ Suggested Action: Ensure the argument matches the expected type. Double-check your function call for the correct data type!\n\n\
                    Code:\n   => {}",
                    line_number,
                    line_number,
                    nm,
                    expected_type,
                    provided_type,
                    line
                ));
            }
        }

        return Ok(Tokens::FnCall(nm.to_string(), provided_args));
    }

    Err(format!(
        "✘ Error: Unable to Parse at Line {}\n\
        At line {}, I couldn't parse the line—it seems jumbled!\n\
        ➔ Reason: The syntax may not be correct. Let's ensure it follows the expected format!\n\
        ➔ Suggested Action: Review your syntax to ensure all components are in order.\n\n\
        Code:\n   => {}",
        line_number, line_number, line
    ))
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
        Err(format!("Could not determine type for argument: {}", arg))
    }
}
