use std::process::exit;

use crate::utils::case::process_case;

use super::{
    cond_evaluator::eval_cond,
    tokens::{func::process_func, input::process_input, print::process_print, var::process_var},
    types::{Args, Tokens, Vars},
};

#[allow(unused, irrefutable_let_patterns)]
pub fn gentoken(mut code: Vec<String>, casetkns: Vec<Tokens>, fc: bool) -> Result<Vec<Tokens>, String> {
    let mut index: i64 = 0;
    let mut tokens: Vec<Tokens> = casetkns;
    let mut ct: Vec<Tokens> = Vec::new();
    let mut in_function = false;
    let mut function_body = Vec::new();
    let mut brace_depth = 0;
    let mut p_label = 0;
    let mut fp_label = 364;
    let mut incase = false;
    let mut cbody: Vec<String> = Vec::new();
    let mut cname = String::new();
    let mut inif = false;
    let mut ifbody: Vec<String> = Vec::new();
    let mut lastfnd: bool = false;
    let mut ctypecond = false;

    for (mut i, mut ln) in code.clone().iter().enumerate() {
        // if i != 0{

        //     code.remove(i-1);
        // }
        //println!("ln : {:?} | inif : {:?} | incase : {:?}", ln, inif, incase);
        let mut ln = ln.as_str();
        ln = ln.trim(); // Trim whitespace from the line
        if let Some(pos) = ln.find('#') {
            ln = ln[..pos].trim(); // Remove comments
        }
        index += 1;

        if ln.starts_with("if{") && !in_function {
            inif = true;
            brace_depth += 1;
            continue;
        } else if inif && !in_function {
            if ln != "}" {
                let pts: Vec<&str> = ln.split(":").collect();
                if pts.len() != 2 && !ln.trim().is_empty() {
                    return Err(format!(
                        "Error at line '{}':\n\
                        An 'if' statement must have 2 parts separated by ':'.\n\
                        You provided: {}\n\
                        Please fix this!",
                        index, ln
                    ));
                }
                let cond = pts[0];
                if cond != "last" {
                    if !ctypecond {
                        let a = eval_cond(cond, &tokens);
                        match a {
                            Ok(k) => {
                                println!("cond : {}", k);
                                println!("ctc -> {}", ctypecond);
                                println!("to c!");
                                match eval_cond(&k, &tokens) {
                                    Ok(cc) => {
                                        println!("cc -> {}", cc);
                                        ifbody.push(format!("{}:{}", cc, pts[1]));
                                    }
                                    Err(e) => return Err(format!("{}", e)),
                                }
                            }
                            Err(e) => {
                                eprintln!("error at line {}\n{}", index, e);
                                exit(1);
                            }
                        }
                    } else {
                        ifbody.push(format!("{}:{}", cond, pts[1]));
                    }
                } else {
                    if lastfnd != true {
                        ifbody.push(format!("{}:{}", cond, pts[1]));
                    } else {
                        return Err(format!("Last condition already evaluated!"));
                    }
                }
                if brace_depth != 0{

                    brace_depth -= 1;
                }

                //ifbody.push(ln.to_string());
            } else {
                inif = false;
                //println!("ifbod : \n{:?}", ifbody);
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
                cbody.push(ln.to_string());
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
                    "{} Error: Nested Function Detected!\n\
                    You're trying to define a new function at line {} while still inside another function.\n\
                    What happened:\n\
                    → At line {}, you started a new function without closing the previous one.\n\
                    What to do:\n\
                    1. Complete the current function definition.\n\
                    2. Ensure each function has matching braces ('{{' and '}}').\n\
                    Code snippet causing the issue:\n\
                    ----------------------------\n\
                    {}\n\
                    ----------------------------\n\
                    Please close the current function before starting a new one!",
                    "✘", index, index, ln
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
            //println!("\nadding ln to func : \n{}", ln);
            //println!("brace depth before : {}", brace_depth);
            if brace_depth == 0{
                
            } 
            function_body.push(ln);
            brace_depth += ln.matches('{').count();
            brace_depth -= ln.matches('}').count();
            //println!("brace depth after : {}", brace_depth);
            if brace_depth > 0 && (i + 1 >= code.len()) {
                return Err(format!(
                    "{} Error: Function has not been properly closed!\n\
                    The function declaration is missing a closing brace.\n\
                    What happened:\n\
                    → At line {}, there is an unmatched opening brace '{{'. The function remains open and needs to be properly closed.\n\
                    What to do:\n\
                    1. Ensure that every opening brace '{{' has a corresponding closing brace '}}'.\n\
                    2. Check the function body for completeness.\n\
                    Code snippet causing the issue:\n\
                    ----------------------------\n\
                    {}\n\
                    ----------------------------\n\
                    Please fix the braces to ensure proper function closure!",
                    "✘", i, code.join("\n")
                ));
            }

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
                                "{} Error: Function Name Already Declared!\n\
                                The function '{}' was declared at line {}. Function names must be unique within the same scope.\n\
                                What happened:\n\
                                → At line {}, you tried to declare the function '{}', but it already exists.\n\
                                What to do:\n\
                                1. Choose a unique name for the new function.\n\
                                2. Ensure the name is descriptive and relevant.\n\
                                Code snippet causing the issue:\n\
                                ----------------------------\n\
                                {}\n\
                                ----------------------------\n\
                                Let’s keep it unique!",
                                "✘", func.name, index, index, func.name, full_function_code
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
                    "Error: Invalid Case Name at Line {} in Main File\n\
                    Case name '{}' contains invalid characters. Only alphabetic characters (A-Z, a-z) and underscores ('_') are allowed.\n\
                    Provided: '{}'.\n\
                    What to do:\n\
                    1. Ensure the case name consists only of letters and underscores.\n\
                    2. Avoid numbers, special characters, or spaces.\n\
                    Once corrected, we can proceed!",
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
        } else if ln.to_ascii_lowercase() == "[c]" {
            ctypecond = true;
        } else if ln.to_ascii_lowercase() == "![c]" {
            ctypecond = false;
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
                            "{} Error: Incorrect Number of Arguments at Line {}\n\
                            Function '{}' called with the wrong number of arguments.\n\
                            → Expected: {} arguments\n\
                            → Provided: {}\n\
                            → Line {}: Attempted to call '{}'. Expected {}, but got {}.\n\
                            Suggestions:\n\
                            1. Check the function definition for the correct number of arguments.\n\
                            2. Adjust the call to match the expected number.\n\
                            Code:\n\
                            ----------------------------\n\
                            {}\n\
                            ----------------------------\n\
                            Let’s fix this!",
                            "✘",
                            index,
                            nm.trim(),
                            expected_args.len(),
                            provided_args.len(),
                            index,
                            nm.trim(),
                            expected_args.len(),
                            provided_args.len(),
                            ln
                        ));
                    }

                    for (provided, expected) in provided_args.iter().zip(expected_args.iter()) {
                        let provided_type = match determine_type(provided, &tokens) {
                            Ok(t) => t,
                            Err(_) => {
                                return Err(format!(
                                    "{} Error: Unable to Parse Argument at Line {}\n\
                                    Trouble parsing argument '{}'.\n\
                                    → Line {}: Argument '{}' couldn’t be processed.\n\
                                    Suggestions:\n\
                                    1. Check syntax, data type, and special characters.\n\
                                    2. Look for typos or misplaced punctuation.\n\
                                    Code:\n\
                                    ----------------------------\n\
                                    {}\n\
                                    ----------------------------\n\
                                    Let’s fix that argument!",
                                    "✘", index, provided, index, provided, ln
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
                                "{} Error: Argument Type Mismatch at Line {}\n\
                                Argument '{}' doesn’t match expected type '{}'.\n\
                                → Line {}: Provided: '{}', Expected: '{}'.\n\
                                Suggestions:\n\
                                1. Ensure argument type aligns with expectations.\n\
                                2. Cast or convert if necessary.\n\
                                3. Check function signature.\n\
                                Code:\n\
                                ----------------------------\n\
                                {}\n\
                                ----------------------------\n\
                                Let’s sort out those types!",
                                "✘",
                                index,
                                provided,
                                expected_type,
                                index,
                                provided,
                                expected_type,
                                ln
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
                } else if ln.ends_with(";") {
                    return Err(format!(
                        "{} Error: Syntax Conundrum at Line {}\n\
                        It seems you’ve ended your line with a semicolon! Here’s what to check:\n\
                        → Line {}:\n\
                        → {}\n\
                        Suggestions:\n\
                        1. Did you mean to end your statement? Maybe you intended to continue?\n\
                        2. Is this a single statement or something more? Reflect on your semicolon’s role.\n\
                        3. Watch for typos or formatting errors that might mislead me!\n\
                        Just a nudge to help you along! You’ve got this!",
                        "!", index, index, ln
                    ));
                }
                return Err(format!(
                    "{} Error: Syntax Issue at Line {}\n\
                    There seems to be a syntax error at line {}:\n\
                    → {}\n\
                    Suggestions:\n\
                    1. Check for missing punctuation or keywords.\n\
                    2. Ensure all brackets and quotes are matched.\n\
                    3. Look for typos or unusual formatting.\n\
                    A quick review can help us resolve this!",
                    "✘", index, index, ln
                ));
            }
        }
    }
    //println!("ct :\n{:?}\ntokens : \n{:?}", ct, tokens);
    drop(code);
    drop(cbody);

    drop(ifbody);
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
