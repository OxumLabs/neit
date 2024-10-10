use crate::utils::{
    case::process_case,
    ftokens::parse_single_line,
    types::{fvars, Args, Tokens, FN},
};

#[allow(unused)]
pub fn process_func(ln: &str, index: usize, p_label: &mut i32) -> Result<FN, String> {
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
    let mut cbody: Vec<&str> = Vec::new();
    let mut cname = String::new();
    let mut brace_depth = 0;
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
                    "✘ Error: Invalid Argument Declaration\n\n\
                    Yikes! Invalid argument declaration at line {}.\n\n\
                    ➔ Hint: Arguments should be in the format 'name:type'.\n\
                    ➔ Example: A valid declaration would look like 'myArg:int' or 'count:float'.\n\n\
                    ⚙ [Code: {}]",
                    index as i32, ln
                ));
            }
            let (name, t) = (pts[0], pts[1]);
            if name.is_empty() {
                return Err(format!(
                    "✘ Error: Missing Argument Name\n\n\
                    Yikes! Argument name is missing at line {}.\n\n\
                    ➔ Hint: Please provide a valid argument name to ensure proper function declaration.\n\
                    ➔ Example: An argument should look like 'argName:int' or 'paramName:string'.\n\n\
                    ⚙ [Code: {}]",
                    index as i32, ln
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
                        "✘ Error: Unrecognized Argument Type\n\n\
                        Yikes! I found an unrecognized argument type '{}' at line {}.\n\n\
                        ➔ What Happened: This means that the type you used is not one of the recognized types in the language.\n\
                        ➔ Supported Types: You can use the following types:\n\
                            - 'string': Used for text values, like \"Hello, World!\"\n\
                            - 'int': Used for whole numbers, like 42 or -10.\n\
                            - 'float': Used for decimal numbers, like 3.14 or -0.5.\n\n\
                        ➔ Example: If you intended to declare a variable, it should look like this:\n\
                            - For a string: 'name:string'\n\
                            - For an integer: 'age:int'\n\
                            - For a float: 'price:float'\n\n\
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
        if incase {
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

            cbody.push(ln);
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
                    "✘ Error: Incorrect Function Declaration Format\n\n\
                    Yikes! I noticed that the function declaration format is incorrect at line {}.\n\n\
                    ➔ What Happened: This means that the way you declared your function doesn't follow the expected structure.\n\
                    ➔ Correct Format: Make sure your function is declared like this:\n\
                        'pub fn functionName(arg1:type, arg2:type)'\n\
                        - 'pub': This means the function is public and can be accessed from other parts of the program.\n\
                        - 'fn': This keyword indicates that you are declaring a function.\n\
                        - 'functionName': Replace this with the name you want to give your function.\n\
                        - 'arg1:type, arg2:type': List your arguments in parentheses, with each argument followed by its type. You can have multiple arguments separated by commas.\n\n\
                    ➔ Example: A valid declaration would look like:\n\
                        'pub fn add(x:int, y:int)'\n\
                    This declares a function named 'add' that takes two integers.\n\n\
                    ⚙ [Code: {}]",
                    index as i32, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){}"));
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
                    "✘ Error: Incorrect Function Declaration Format\n\n\
                    Yikes! I found an issue with the function declaration format at line {}.\n\n\
                    ➔ What Happened: This means that the way you wrote your function declaration doesn't match the expected format.\n\
                    ➔ Correct Format: Ensure your function is declared like this:\n\
                        'fn functionName(arg1:type, arg2:type)'\n\
                        - 'fn': This keyword indicates that you are declaring a function.\n\
                        - 'functionName': Replace this with the name you want for your function.\n\
                        - 'arg1:type, arg2:type': List your arguments inside parentheses, where each argument has a name followed by its type. You can have multiple arguments separated by commas.\n\n\
                    ➔ Example: A valid declaration would look like:\n\
                        'fn add(x:int, y:int)'\n\
                    This declares a function named 'add' that takes two integer arguments.\n\n\
                    ⚙ [Code: {}]",
                    index as i32, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){}"));
            functions.name = name.to_string();
            functions.is_global = false;
            if !arg.trim().is_empty() {
                parse_arguments(arg, &mut functions, &mut fnbody, &mut lv, ln, index)?;
            }
        } else if ln.starts_with("fn ") && ln.ends_with("{") {
            if inf {
                return Err(format!(
                    "✘ Error: Nested Function Definitions Not Allowed\n\n\
                    Yikes! I found a nested function definition at line {}.\n\n\
                    ➔ What Happened: This means that you tried to define a new function inside another function, which is not permitted.\n\
                    ➔ Solution: Make sure to close the previous function before you start defining a new one. Each function should be separate and properly defined.\n\n\
                    ➔ Example: Here’s how you can correctly define functions:\n\
                        1. Define the first function:\n\
                            'fn outerFunction() {{\n\
                                // Code for the outer function\n\
                            }}'\n\
                        2. Then, define a separate function:\n\
                            'fn anotherFunction() {{\n\
                                // Code for the second function\n\
                            }}'\n\
                    ➔ Remember: Functions should not be nested within each other!\n\n\
                    ⚙ [Code: {}]",
                    index, ln
                ));
            }
            let pts: Vec<&str> = ln[3..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "✘ Error: Incorrect Function Declaration Format\n\n\
                    Yikes! I found an issue with the function declaration format at line {}.\n\n\
                    ➔ What Happened: The way you wrote your function declaration doesn't match the expected format.\n\
                    ➔ Correct Format: Ensure your function is declared like this:\n\
                        'fn functionName(arg1:type){{'\n\
                        - 'fn': This keyword indicates that you are declaring a function.\n\
                        - 'functionName': Replace this with the name you want for your function.\n\
                        - 'arg1:type': This is where you specify the argument name followed by its type.\n\
                        - '{{': Don’t forget to open the function body with a curly brace!\n\n\
                    ➔ Example: A valid declaration would look like:\n\
                        'fn add(x:int){{'\n\
                    This declares a function named 'add' that takes one integer argument and opens its body.\n\n\
                    ⚙ [Code: {}]",
                    index as i32, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){"));
            functions.name = name.to_string();
            functions.is_global = false;

            if !arg.trim().is_empty() {
                parse_arguments(arg, &mut functions, &mut fnbody, &mut lv, ln, index)?;
            }
            inf = true;
        } else if ln.starts_with("pub fn ") && ln.ends_with("{") {
            if inf {
                return Err(format!(
                    "✘ Error: Nested Function Definitions Not Allowed\n\n\
                    Yikes! I found a nested function definition at line {}.\n\n\
                    ➔ What Happened: You attempted to define a new function inside another function. This is not allowed in the current programming rules.\n\
                    ➔ Solution: To fix this, you need to close the previous function before starting a new one. Each function must be defined separately and clearly.\n\n\
                    ➔ Example of Correct Function Definitions:\n\
                        1. Define the first function:\n\
                            'fn outerFunction() {{\n\
                                // Code for the outer function\n\
                            }}'\n\
                        2. Then, define a separate function:\n\
                            'fn anotherFunction() {{\n\
                                // Code for the second function\n\
                            }}'\n\
                    ➔ Remember: Always close a function before defining another to keep your code organized!\n\n\
                    ⚙ [Code: {}]",
                    index, ln
                ));
            }
            let pts: Vec<&str> = ln[7..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "✘ Error: Incorrect Function Declaration Format\n\n\
                    Yikes! There’s a problem with the function declaration format at line {}.\n\n\
                    ➔ What Happened: The way you wrote your function declaration doesn’t match the expected format.\n\
                    ➔ Correct Format: Make sure your function is declared like this:\n\
                        'pub fn functionName(arg1:type){{'\n\
                        - 'pub': This keyword indicates that the function is public and can be accessed from other parts of the program.\n\
                        - 'fn': This keyword signifies that you are declaring a function.\n\
                        - 'functionName': Replace this with the name you want for your function.\n\
                        - 'arg1:type': This is where you specify the argument name followed by its type.\n\
                        - '{{': Don’t forget to open the function body with a curly brace!\n\n\
                    ➔ Example: A valid declaration might look like:\n\
                        'pub fn add(x:int){{'\n\
                    This declares a public function named 'add' that takes one integer argument and opens its body.\n\n\
                    ⚙ [Code: {}]",
                    index as i32, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){"));
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
                                "✘ Error: Unexpected Issue Encountered\n\n\
                                Yikes! An unexpected error occurred at line {}.\n\n\
                                ➔ What Happened: There was an error in your code that I wasn't prepared for. This can happen due to various reasons, such as incorrect syntax, unsupported operations, or misconfigured settings.\n\
                                ➔ Suggested Action: Please review the syntax around this line carefully and ensure everything is correct. Double-check for common mistakes like missing brackets, incorrect function names, or unsupported expressions.\n\n\
                                ➔ Here’s the specific error message for more context:\n\
                                    [Error: {}]\n\n\
                                ⚙ [Code: {}]",
                                index, e, ln
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
                "✘ Error: Unrecognized Line Format Detected\n\n\
                Yikes! I found an unrecognized line format at line {}.\n\n\
                ➔ What Happened: The line may not conform to the expected syntax for function declarations or code blocks. This can occur due to missing keywords, incorrect punctuation, or other syntax errors.\n\
                ➔ Suggested Action: Please review the line carefully and ensure that it follows the proper syntax rules for function declarations or the body of your code. Common things to check include:\n\
                    - Ensure the correct use of keywords like 'fn' for function declarations.\n\
                    - Check that parentheses and curly braces are used properly.\n\
                    - Make sure the structure of your function or statement matches what is expected.\n\n\
                ➔ Here’s the specific line that caused the issue:\n\
                ⚙ [Code: {}]\n\
                Let’s fix that format and keep your code running smoothly!",
                index, ln
            ));
        }
    }

    if inf {
        return Err(format!(
            "✘ Error: Open Function Body Detected\n\n\
            Yikes! I found an open function body that is not properly closed.\n\n\
            ➔ What Happened: It seems you have started a function but haven't provided a closing brace '}}'. This means the function is left open, which can lead to confusion and potential errors in your code.\n\
            ➔ Suggested Action: Please check your code and ensure that every function you declare has a matching closing brace. Here are a few things to verify:\n\
                - Make sure every '{{' has a corresponding '}}'.\n\
                - If you’ve nested functions, ensure that each inner function is closed before closing the outer one.\n\
                - It might help to use consistent indentation to easily spot where functions open and close.\n\n\
            Let’s tidy up the function closures to keep your code clean and organized!"
        ));
    }

    functions.local_vars = lv;
    println!("funcs : \n{:?}", functions);
    Ok(functions)
}
