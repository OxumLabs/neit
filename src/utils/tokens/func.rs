use crate::utils::{
    case::process_case,
    ftokens::parse_single_line,
    tokens::var::C_KEYWORDS,
    types::{fvars, Args, Tokens, FN},
};

#[allow(unused)]
pub fn process_func(ln: &str, index: usize, p_label: &mut i32) -> Result<FN, String> {
    //println!("ln : {}", ln);
    let mut functions = FN::new(
        "_NAME_".to_string(),
        false,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let mut inf = false;
    let mut fnbody: Vec<Tokens> = Vec::new();
    let lines: Vec<&str> = ln.trim().split("\n").collect();
    let mut lv: Vec<fvars> = Vec::new();

    let mut incase = false;
    let mut cbody: Vec<String> = Vec::new();
    let mut cname = String::new();
    let mut brace_depth = 0;

    let mut inif = false;
    let mut ifbody: Vec<String> = Vec::new();

    // Helper function to parse arguments
    fn parse_arguments(
        arg_str: &str,
        functions: &mut FN,
        fnb: &mut Vec<Tokens>,
        lv: &mut Vec<fvars>,
        ln: &str,
        index: usize,
    ) -> Result<(), String> {
        let args = arg_str.split(",").map(str::trim);
        for i in args {
            let pts: Vec<&str> = i.split(":").map(str::trim).collect();
            if pts.len() != 2 && !ln.trim().is_empty() {
                return Err(format!(
                    "✘ Error: Invalid Argument Declaration\n\
                    Invalid argument declaration at line {}.\n\
                    ➔ Hint: Use the format 'name:type' (e.g., 'myArg:int').\n\
                    ⚙ Code: \n{}",
                    index as i32, ln
                ));
            }
            let (name, t) = (pts[0], pts[1]);
            if name.is_empty() {
                return Err(format!(
                    "✘ Error: Missing Argument Name\n\
                    Argument name is missing at line {}.\n\
                    ➔ Hint: Provide a valid argument name (e.g., 'argName:int').\n\
                    ⚙ Code: \n{}",
                    index as i32, ln
                ));
            }
            if C_KEYWORDS.contains(&name) {
                return Err(format!(
                    "✘ Error:  Invalid Argument Name '{}' at {}\n\
                    Oops! This name is a reserved C keyword. \n\
                    ➔ Reason: Keywords have special meanings and cannot be used as variable names. \n\
                    ➔ Suggested Action: Change the variable name to avoid conflicts. \n\
                    ➔ Example: Instead of 'char', use 'char_var'.",
                    name,index
                ));
            }
            match t.to_lowercase().as_str() {
                "string" => {
                    lv.push(fvars {
                        v: crate::utils::types::Vars::STR(String::new()),
                        n: name.to_string(),
                    });
                    fnb.push(Tokens::Var(
                        crate::utils::types::Vars::STR(String::new()),
                        name.to_string(),
                        false,
                    ));
                    functions.args.push(Args::Str(name.to_string()));
                }
                "int" => {
                    lv.push(fvars {
                        v: crate::utils::types::Vars::INT(0),
                        n: name.to_string(),
                    });
                    fnb.push(Tokens::Var(
                        crate::utils::types::Vars::INT(0),
                        name.to_string(),
                        false,
                    ));
                    functions.args.push(Args::Int(name.to_string()));
                }
                "float" => {
                    lv.push(fvars {
                        v: crate::utils::types::Vars::F(0.0),
                        n: name.to_string(),
                    });
                    fnb.push(Tokens::Var(
                        crate::utils::types::Vars::F(0.0),
                        name.to_string(),
                        false,
                    ));
                    functions.args.push(Args::Float(name.to_string()));
                }
                _ => {
                    return Err(format!(
                        "✘ Error: Unrecognized Argument Type\n\
                        Found unrecognized type '{}' at line {}.\n\
                        ➔ Supported Types: 'string', 'int', 'float'.\n\
                        ➔ Example: Declare as 'name:string', 'age:int', or 'price:float'.\n\
                        ⚙ [Code: {}]",
                        t, index, ln
                    ));
                }
            }
        }
        Ok(())
    }

    for ln in lines {
        let ln = ln.trim();
        //println!("ln : {}", ln);
        if ln.starts_with("if{") {
            inif = true;
            continue;
        } else if inif {
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

                ifbody.push(ln.to_string());
            } else {
                inif = false;
                let iftkn = Tokens::Cond(ifbody.clone());

                fnbody.push(iftkn);
                continue;
            }
        } else if incase {
            brace_depth += ln.matches("{").count();
            brace_depth -= ln.matches("}").count();
            if brace_depth == 0 {
                incase = false;
                let pc = process_case(ln, cbody.clone(), &mut (index as i64), &fnbody, true);
                match pc {
                    Ok(k) => {
                        fnbody.push(Tokens::IFun(cname.clone(), k.clone()));
                        //println!("k : {:?}\ntokens : \n{:?}", k, fnbody);
                    }
                    Err(e) => return Err(e),
                }
                // println!("cbody : {:?}", cbody);
            }

            cbody.push(ln.to_owned());
        } else if ln.starts_with("case ") && ln.ends_with("{") {
            let cnamee = ln[5..].trim_end_matches("{");
            if !cname.chars().all(|c| c.is_alphabetic() || c == '_') {
                return Err(format!(
                    "✘ Error: Invalid Identifier Found\n\n\
                    Error at line '{}' in the main file.\n\n\
                    ➔ Reason: {} names can only contain alphabets and underscores ('_'). You provided '{}'.\n\n\
                    ➔ Action: Please fix the identifier and then we can continue!\n\n\
                    ➔ Hint: Make sure your identifier starts with a letter and is followed by letters, digits, or underscores.",
                    index,
                    "Case",
                    cname
                ));
            }
            cname = cnamee.to_string();
            brace_depth += 1;
            incase = true;
        } else if ln.starts_with("pub fn ") && ln.ends_with("{}") {
            let pts: Vec<&str> = ln[7..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "✘ Error: Incorrect Function Declaration Format\n\
                    Found an incorrect function declaration at line {}.\n\
                    ➔ Correct Format: 'pub fn functionName(arg1:type, arg2:type)'.\n\
                    - 'pub': Public access.\n\
                    - 'fn': Indicates a function.\n\
                    - 'functionName': Your function's name.\n\
                    - 'arg1:type, arg2:type': Arguments with types, separated by commas.\n\
                    ➔ Example: 'pub fn add(x:int, y:int)' declares a function 'add' taking two integers.\n\
                    ⚙ [Code: {}]",
                    index, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){}"));
            if C_KEYWORDS.contains(&name) {
                return Err(format!(
                    "✘ Error:  Invalid Function Name '{}' at line {}\n\
                    Oops! This name is a reserved C keyword. \n\
                    ➔ Reason: Keywords have special meanings and cannot be used as variable names. \n\
                    ➔ Suggested Action: Change the variable name to avoid conflicts. \n\
                    ➔ Example: Instead of 'char', use 'char_var'.",
                    name,index
                ));
            }
            functions.name = name.to_string();
            functions.is_global = true;
            if !arg.trim().is_empty() {
                parse_arguments(arg, &mut functions, &mut fnbody, &mut lv, ln, index)?;
            } else {
                functions.args.push(Args::EMP("_".to_string()));
            }
        } else if ln.starts_with("fn ") && ln.ends_with("{}") {
            let pts: Vec<&str> = ln[3..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "✘ Error: Incorrect Function Declaration Format\n\
                    Found an issue with the function declaration at line {}.\n\
                    ➔ What Happened: The function declaration doesn't match the expected format.\n\
                    ➔ Correct Format: 'fn functionName(arg1:type, arg2:type)'.\n\
                    - 'fn': Indicates a function.\n\
                    - 'functionName': Your function's name.\n\
                    - 'arg1:type, arg2:type': Arguments in parentheses with names followed by types, separated by commas.\n\
                    ➔ Example: 'fn add(x:int, y:int)' declares a function 'add' taking two integers.\n\
                    ⚙ Code: \n{}",
                    index, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){}"));
            if C_KEYWORDS.contains(&name) {
                return Err(format!(
                    "✘ Error:  Invalid Function Name Name '{}'\n\
                    Oops! This name is a reserved C keyword. \n\
                    ➔ Reason: Keywords have special meanings and cannot be used as variable names. \n\
                    ➔ Suggested Action: Change the variable name to avoid conflicts. \n\
                    ➔ Example: Instead of 'char', use 'char_var'.",
                    name
                ));
            }
            functions.name = name.to_string();
            functions.is_global = false;
            if !arg.trim().is_empty() {
                parse_arguments(arg, &mut functions, &mut fnbody, &mut lv, ln, index)?;
            }
        } else if ln.starts_with("fn ") && ln.ends_with("{") {
            if inf {
                return Err(format!(
                    "✘ Error: Nested Function Definitions Not Allowed\n\
                    Found a nested function definition at line {}.\n\
                    ➔ What Happened: You tried to define a function inside another function, which is not allowed.\n\
                    ➔ Solution: Close the previous function before starting a new one. Functions must be separate and properly defined.\n\
                    ➔ Example:\n\
                        1. First function:\n\
                            'fn outerFunction() {{ /* Code */ }}'\n\
                        2. Second function:\n\
                            'fn anotherFunction() {{ /* Code */ }}'\n\
                    ➔ Remember: Functions cannot be nested!\n\
                    ⚙ [Code: {}]",
                    index, ln
                ));
            }
            let pts: Vec<&str> = ln[3..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "✘ Error: Incorrect Function Declaration Format\n\
                    Found an issue with the function declaration format at line {}.\n\
                    ➔ What Happened: The function declaration doesn't match the expected format.\n\
                    ➔ Correct Format: Declare your function like this:\n\
                        'fn functionName(arg1:type){{'\n\
                        - 'fn': Keyword for declaring a function.\n\
                        - 'functionName': Name of your function.\n\
                        - 'arg1:type': Argument name followed by its type.\n\
                        - '{{': Open the function body with a curly brace!\n\
                    ➔ Example: A valid declaration:\n\
                        'fn add(x:int){{'\n\
                    This defines a function 'add' that takes one integer argument and opens its body.\n\
                    ⚙ [Code: {}]",
                    index as i32, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){"));
            if C_KEYWORDS.contains(&name) {
                return Err(format!(
                    "✘ Error:  Invalid Function Name Name '{}' at line '{}'\n\
                    Oops! This name is a reserved C keyword. \n\
                    ➔ Reason: Keywords have special meanings and cannot be used as variable names. \n\
                    ➔ Suggested Action: Change the variable name to avoid conflicts. \n\
                    ➔ Example: Instead of 'char', use 'char_var'.",
                    name,index
                ));
            }
            functions.name = name.to_string();
            functions.is_global = false;

            if !arg.trim().is_empty() {
                parse_arguments(arg, &mut functions, &mut fnbody, &mut lv, ln, index)?;
            }
            inf = true;
        } else if ln.starts_with("pub fn ") && ln.ends_with("{") {
            if inf {
                return Err(format!(
                    "✘ Error: Nested Function Definitions Not Allowed\n\
                    Found a nested function definition at line {}.\n\
                    ➔ What Happened: You tried to define a new function inside another function, which is not allowed.\n\
                    ➔ Solution: Close the previous function before starting a new one. Each function must be defined separately.\n\
                    ➔ Example of Correct Definitions:\n\
                        1. Define the first function:\n\
                            'fn outerFunction() {{\n\
                                // Code for the outer function\n\
                            }}'\n\
                        2. Then define another:\n\
                            'fn anotherFunction() {{\n\
                                // Code for the second function\n\
                            }}'\n\
                    ➔ Remember: Close a function before defining another for better organization!\n\
                    ⚙ Code: \n{}",
                    index, ln
                ));
            }
            let pts: Vec<&str> = ln[7..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "✘ Error: Incorrect Function Declaration Format\n\
                    Yikes! There's a problem with the function declaration at line {}.\n\
                    ➔ What Happened: The function declaration doesn't match the expected format.\n\
                    ➔ Correct Format: Ensure your function is declared like this:\n\
                        'pub fn functionName(arg1:type){{'\n\
                        - 'pub': Indicates the function is public.\n\
                        - 'fn': Signifies that you're declaring a function.\n\
                        - 'functionName': Replace this with your desired function name.\n\
                        - 'arg1:type': Specify the argument name followed by its type.\n\
                        - '{{': Open the function body with a curly brace!\n\
                    ➔ Example: A valid declaration might be:\n\
                        'pub fn add(x:int){{'\n\
                    This declares a public function named 'add' that takes one integer argument.\n\
                    ⚙ [Code: {}]",
                    index as i32, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){"));
            if C_KEYWORDS.contains(&name) {
                return Err(format!(
                    "✘ Error:  Invalid Function Name Name '{}' at line '{}'\n\
                    Oops! This name is a reserved C keyword. \n\
                    ➔ Reason: Keywords have special meanings and cannot be used as variable names. \n\
                    ➔ Suggested Action: Change the variable name to avoid conflicts. \n\
                    ➔ Example: Instead of 'char', use 'char_var'.",
                    name,index
                ));
            }
            functions.name = name.to_string();
            functions.is_global = true;
            if !arg.trim().is_empty() {
                parse_arguments(arg, &mut functions, &mut fnbody, &mut lv, ln, index)?;
            }
            inf = true;
        } else if inf {
            if ln == "}" || ln.ends_with("}") {
                functions.code = fnbody.clone();
                inf = false;
            } else {
                let ptkn = parse_single_line(
                    ln.trim(),
                    index,
                    p_label,
                    &mut lv,
                    &mut fnbody,
                    &functions.args,
                );

                match ptkn {
                    Ok(tkn) => match tkn {
                        Tokens::Var(v, n, _) => {
                            lv.push(fvars { v, n });
                        }
                        _ => {
                            fnbody.push(tkn);
                        }
                    },
                    Err(e) => match e.as_str() {
                        "|_EMP_|" => continue,
                        _ => {
                            return Err(format!(
                                "✘ Error: Unexpected Issue Encountered\n\
                                Yikes! An unexpected error occurred at line {}.\n\
                                ➔ Error: \n{}\n",
                                index, e
                            ));
                        }
                    },
                }
            }
        } else if ln.is_empty() {
            continue;
            return Err(format!(
                "✘ Error: Unexpected Empty Line Detected\n\n\
                Yikes! I encountered an unexpected empty line at line {}.\n\n\
                ➔ What Happened: Empty lines can make your code look unclear or messy. While they can sometimes be useful for separating sections of code, too many can lead to confusion about the flow of your program.\n\
                ➔ Suggested Action: Please review the code around this line and remove any unnecessary empty lines. Keeping your code tidy will help maintain clarity and make it easier to read.\n\n\
                ➔ Here’s the specific line in question:\n\
                    ⚙ [Code: {}]\n\
                Let’s keep your code clean and organized!",
                index, ln
            ));
        } else {
            return Err(format!(
                "✘ Error: Unrecognized Line Format at Line {}\n\
                ➔ Issue: The line doesn't match the expected syntax for declarations or code blocks. Check for missing keywords or punctuation.\n\
                ➔ Action: Review the line for proper syntax: use 'fn' for functions, ensure parentheses and braces are correct, and match the expected structure.\n\
                ⚙ Code: \n{}\n Fix the format to keep your code running smoothly!",
                index, ln
            ));
        }
    }

    if inf {
        return Err(format!(
            "✘ Error: Open Function Body Detected\n\
            ➔ Issue: An open function body lacks a closing brace '}}'. This can cause confusion and errors in your code.\n\
            ➔ Action: Ensure every function has a matching closing brace. Verify that every '{{' has a corresponding '}}', and if functions are nested, close each inner function before the outer one. Consistent indentation can help track function boundaries.\n\
            Let's tidy up the function closures for cleaner code!"
        ));
    }

    functions.local_vars = lv;
    //println!("funcs : \n{:?}", functions);
    Ok(functions)
}
