use crate::utils::case::process_case;

use super::{
    tokens::{func::process_func, input::process_input, print::process_print, var::process_var},
    types::{Args, Tokens, Vars},
};
use colored::*; // Import the colored crate

#[allow(unused, irrefutable_let_patterns)]
pub fn gentoken(code: Vec<&str>, casetkns: Vec<Tokens>, fc: bool) -> Result<Vec<Tokens>, String> {
    let mut index: i64 = 0;
    let mut tokens: Vec<Tokens> = casetkns;
    let mut ct: Vec<Tokens> = Vec::new();
    let mut in_function = false;
    let mut function_body = Vec::new();
    let mut brace_depth = 0;
    let mut p_label = 0;
    let mut fp_label = 364;
    let mut incase = false;
    let mut cbody: Vec<&str> = Vec::new();
    let mut cname = String::new();
    let mut inif = false;
    let mut ifbody: Vec<String> = Vec::new();

    for mut ln in code.clone() {
        //println!("ln : {:?} | inif : {:?} | incase : {:?}", ln, inif, incase);
        ln = ln.trim(); // Trim whitespace from the line
        if let Some(pos) = ln.find('#') {
            ln = ln[..pos].trim(); // Remove comments
        }
        index += 1;

        if ln.starts_with("if{") {
            inif = true;
            continue;
        } else if inif {
            if ln != "}" {
                let pts: Vec<&str> = ln.split(":").collect();
                if pts.len() != 2 && !ln.trim().is_empty() {
                    return Err(format!("Error at line '{}'\nConditions given to 'if' shall have 2 parts separated by ':'\nfirst part is conditions nd second is case to call\nhere you gave me this : {}\nwhat is this? fix this right now!",index,ln));
                }

                ifbody.push(ln.to_string());
            } else {
                inif = false;
                println!("ifbod : \n{:?}", ifbody);
                let iftkn = Tokens::Cond(ifbody.clone());
                if fc {
                    ct.push(iftkn);
                } else {
                    tokens.push(iftkn);
                }
            }
        } else if incase {
            brace_depth += ln.matches("{").count();
            brace_depth -= ln.matches("}").count();
            if brace_depth == 0 {
                incase = false;
                // println!("cname : {}\ncbody : {:?}", cname, cbody);
                let pc = process_case(ln, cbody.clone(), &mut index, &tokens, true);
                cbody.clear();
                match pc {
                    Ok(k) => {
                        if fc {
                            ct.push(Tokens::IFun(cname.clone(), k.clone()));
                        } else {
                            tokens.push(Tokens::IFun(cname.clone(), k.clone()));
                        }
                        //println!("k : {:?}\ntokens : \n{:?}", k, tokens);
                    }
                    Err(e) => return Err(e),
                }
                //println!("cbody : {:?}", cbody);
            } else {
                cbody.push(ln);
            }
        }

        if ln.is_empty() {
            continue; // Skip empty lines
        }

        // Handle function definitions
        if (ln.trim().starts_with("pub fn") || ln.trim().starts_with("fn "))
            && ln.trim().ends_with("{")
        {
            if in_function {
                return Err(format!(
                    "{} Error: Nested Function Detected!\n\n\
                    It looks like you're trying to define a new function at line {} while you're still inside an open function.\n\
                    This can cause unexpected behavior since functions cannot overlap.\n\
                    Here's what happened:\n\n\
                    → At line {}, you attempted to start another function definition.\n\
                    → But you haven't properly closed the previous function.\n\n\
                    What you should do:\n\
                    1. Complete the function you started earlier.\n\
                    2. Ensure each function has a clear start and end with matching braces ('{{' and '}}').\n\n\
                    Code snippet causing the issue:\n\
                    ----------------------------\n\
                    {}\n\
                    ----------------------------\n\n\
                    Please close the current function properly before beginning a new one.",
                    "✘".red(), index, index, ln
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
                                "{} Error: Function Name Already Declared!\n\n\
                                The function '{}' was already declared at line {}. Function names must be unique within the same scope.\n\
                                Re-declaring a function with the same name can lead to unexpected behavior and confusion when referencing the correct function.\n\n\
                                Here's what happened:\n\n\
                                → At line {}, you tried to declare the function '{}', but a function with this name already exists.\n\
                                → Each function should have a unique and descriptive name.\n\n\
                                What you should do:\n\
                                1. Choose a unique name for the new function.\n\
                                2. Ensure that the name is descriptive and relevant to its purpose.\n\n\
                                Code snippet causing the issue:\n\
                                ----------------------------\n\
                                {}\n\
                                ----------------------------\n\n\
                                Let’s keep it unique and avoid naming collisions!",
                                "✘".red(), func.name, index, index, func.name, full_function_code
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
            let cnamee = ln[5..].trim_end_matches("{");
            if !cname.chars().all(|c| c.is_alphabetic() || c == '_') {
                return Err(format!(
                    "Error: Invalid Case Name at Line {} in Main File\n\n\
                    It looks like the case name '{}' contains invalid characters.\n\
                    Case names can only contain alphabetic characters (A-Z, a-z) and underscores ('_').\n\
                    You've provided: '{}', which doesn't meet these criteria.\n\n\
                    Here's what you need to do:\n\
                    1. Ensure the case name consists only of letters and underscores.\n\
                    2. Avoid using numbers, special characters, or spaces.\n\n\
                    Once you've corrected the case name, we can proceed!\n",
                    index, cname, cname
                ));
            }
            cname = cnamee.to_string();
            brace_depth += 1;
            incase = true;
        } else if ln.trim().starts_with("must ") && !incase && !inif {
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
        } else if ln.trim().starts_with("may ") && !incase && !inif {
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
        } else if ln.trim().starts_with("print(") && ln.trim().ends_with(")") && !incase && !inif {
            let mut txt: String = ln[6..ln.len() - 1].trim().to_string(); // Extract println arguments
            let ptxt = process_print(&mut p_label, &txt, &tokens);
            if !fc {
                tokens.push(ptxt);
            } else {
                ct.push(ptxt);
            }
        } else if ln.starts_with("takein(") && !incase && !inif {
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
        } else if ln.trim().starts_with("println(") && ln.trim().ends_with(")") && !incase && !inif
        {
            let mut txt: String = ln[9..ln.len() - 2].to_string(); // Extract println arguments
            let txt = format!(r#""{}\n""#, txt);
            let ptxt = process_print(&mut p_label, &txt, &tokens);
            if !fc {
                tokens.push(ptxt);
            } else {
                ct.push(ptxt);
            }
        } else if !incase && !inif {
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
                            "{} Error: Incorrect Number of Arguments at Line {}\n\n\
                            The function '{}' was called with the wrong number of arguments.\n\
                            → Expected: {} arguments\n\
                            → Provided: {}\n\n\
                            Each function must be called with the correct number of arguments based on its definition.\n\n\
                            Here's what happened:\n\n\
                            → At line {}, you attempted to call the function '{}'.\n\
                            → The function expects {} arguments, but you provided {}.\n\n\
                            What you should do:\n\
                            1. Check the function definition and ensure you're passing the right number of arguments.\n\
                            2. Add or remove arguments as needed to match the expected number.\n\n\
                            Code snippet causing the issue:\n\
                            ----------------------------\n\
                            {}\n\
                            ----------------------------\n\n\
                            Let’s fix this and keep the code clean!",
                            "✘".red(), index, nm.trim(), expected_args.len(), provided_args.len(), index, nm.trim(), 
                            expected_args.len(), provided_args.len(), ln
                        ));
                    }

                    for (provided, expected) in provided_args.iter().zip(expected_args.iter()) {
                        let provided_type = match determine_type(provided, &tokens) {
                            Ok(t) => t,
                            Err(_) => {
                                return Err(format!(
                                        "{} Error: Unable to Parse Argument at Line {}\n\n\
                                        I'm having trouble parsing the argument '{}'. It seems there's something wrong with its format or structure.\n\n\
                                        What you should do:\n\n\
                                        → At line {}, the argument '{}' couldn't be processed.\n\
                                        → Double-check the syntax, data type, or any special characters in the argument.\n\n\
                                        Suggestions:\n\
                                        1. Ensure the argument matches the expected type and format.\n\
                                        2. Watch for typos, invalid characters, or misplaced punctuation.\n\n\
                                        Code snippet causing the issue:\n\
                                        ----------------------------\n\
                                        {}\n\
                                        ----------------------------\n\n\
                                        Let’s fix that argument and try again!",
                                        "✘".red(), index, provided, index, provided, ln
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
                                "{} Error: Argument Type Mismatch at Line {}\n\n\
                                The argument '{}' doesn't match the expected type '{}'. This type inconsistency could lead to errors in your program's behavior.\n\n\
                                Here's what happened:\n\n\
                                → At line {}, the argument '{}' was provided.\n\
                                → The function or expression expects an argument of type '{}', but the given argument doesn't match that.\n\n\
                                What you should do:\n\
                                1. Ensure the argument type aligns with what the function or expression expects.\n\
                                2. If necessary, cast or convert the argument to the appropriate type.\n\
                                3. Double-check the function signature or definition to confirm the expected type.\n\n\
                                Code snippet causing the issue:\n\
                                ----------------------------\n\
                                {}\n\
                                ----------------------------\n\n\
                                Let's sort out those types and keep the code flowing!",
                                "✘".red(), index, provided, expected_type, index, provided, expected_type, ln
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
                    "{} Error: Syntax Issue at Line {}\n\n\
                    I encountered a problem parsing your code at line {}. It seems there might be a syntax error that’s preventing me from understanding what you intended.\n\n\
                    Here’s what you should check:\n\n\
                    → At line {}, the code is as follows:\n\
                    → {}\n\n\
                    Suggestions for resolution:\n\
                    1. Carefully review your syntax for any missing punctuation or incorrect keywords.\n\
                    2. Ensure that all opening brackets, parentheses, and quotes have matching closing counterparts.\n\
                    3. Look for any typos or unconventional formatting that could confuse the parser.\n\n\
                    I know it can be a bit of a head-scratcher, but with a quick review, we can get this sorted out!",
                    "✘".red(), index, index,index, ln
                ));
            }
        }
    }
    //println!("ct :\n{:?}\ntokens : \n{:?}", ct, tokens);
    if fc {
        return Ok(ct);
    } else {
        Ok(tokens) // Return the generated tokens
    }
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
            "✘ Error: Invalid Type Provided\n\n\
            It appears that '{}' is not a recognized or valid type. Let's get back on track with our data types!\n\n\
            Valid types include:\n\
            → `int`: for integers\n\
            → `float`: for floating-point numbers\n\
            → `string`: for sequences of characters\n\n\
            Code snippet causing the issue:\n\
            ----------------------------\n\
            {}\n\
            ----------------------------\n\
            Please review your type usage and make sure to stick with the supported types. We can get this sorted out!",
            arg, arg
        ));
    }
}
