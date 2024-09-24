use crate::utils::{
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

    for ln in lines {
        let ln = ln.trim();
        if ln.starts_with("pub fn ") && ln.ends_with("{}") {
            let pts: Vec<&str> = ln[7..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "Error at line {}: Incorrect function declaration format.\nCode:\n   => {}\nHint: Ensure the function is declared as 'fn name(arg1:type, arg2:type)'.",
                    index as i32, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){}"));
            functions.name = name.to_string();
            functions.is_global = true;
            if !arg.trim().is_empty() {
                let args = arg.split(",");
                for i in args {
                    let pts: Vec<&str> = i.split(":").collect();
                    if pts.len() != 2 {
                        return Err(format!(
                            "Error at line {}: Invalid argument declaration.\nCode:\n   => {}\nHint: Ensure that each argument is declared as 'name:type'.",
                            index as i32, ln
                        ));
                    }
                    let (name, t) = (pts[0].trim(), pts[1].trim());
                    if name.is_empty() {
                        return Err(format!(
                            "Error at line {}: Argument name cannot be empty.\nCode:\n   => {}\nHint: Provide a valid name for the argument.",
                            index as i32, ln
                        ));
                    }
                    match t {
                        "string" => functions.args.push(Args::Str(name.to_string())),
                        "int" => functions.args.push(Args::Int(name.to_string())),
                        "float" => functions.args.push(Args::Float(name.to_string())),
                        _ => {
                            return Err(format!(
                                "Error at line {}: Invalid argument type '{}'.\nCode:\n   => {}\nHint: Use 'string', 'int', or 'float' as argument types.",
                                index, t, ln
                            ));
                        }
                    }
                }
            } else {
                functions.args.push(Args::EMP("_".to_string()));
            }
        } else if ln.starts_with("fn ") && ln.ends_with("{}") {
            let pts: Vec<&str> = ln[3..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "Error at line {}: Incorrect function declaration format.\nCode:\n   => {}\nHint: Ensure the function is declared as 'fn name(arg1:type, arg2:type)'.",
                    index as i32, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){}"));
            functions.name = name.to_string();
            functions.is_global = false;
            if !arg.trim().is_empty() {
                let args = arg.split(",");
                for i in args {
                    let pts: Vec<&str> = i.split(":").collect();
                    if pts.len() != 2 {
                        return Err(format!(
                            "Error at line {}: Invalid argument declaration.\nCode:\n   => {}\nHint: Ensure that each argument is declared as 'name:type'.",
                            index as i32, ln
                        ));
                    }
                    let (name, t) = (pts[0].trim(), pts[1].trim());
                    if name.is_empty() {
                        return Err(format!(
                            "Error at line {}: Argument name cannot be empty.\nCode:\n   => {}\nHint: Provide a valid name for the argument.",
                            index as i32, ln
                        ));
                    }
                    match t {
                        "string" => functions.args.push(Args::Str(name.to_string())),
                        "int" => functions.args.push(Args::Int(name.to_string())),
                        "float" => functions.args.push(Args::Float(name.to_string())),
                        _ => {
                            return Err(format!(
                                "Error at line {}: Invalid argument type '{}'.\nCode:\n   => {}\nHint: Use 'string', 'int', or 'float' as argument types.",
                                index, t, ln
                            ));
                        }
                    }
                }
            }
        } else if ln.starts_with("fn ") && ln.ends_with("{") {
            if inf {
                return Err(format!(
                    "Error at line {}: Nested function definitions are not allowed.\nCode:\n   => {}\nHint: Ensure that functions are properly closed before starting a new one.",
                    index, ln
                ));
            }
            let pts: Vec<&str> = ln[3..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "Error at line {}: Incorrect function declaration format.\nCode:\n   => {}\nHint: Ensure the function is declared as 'fn name(arg1:type, arg2:type){{'.",
                    index as i32, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){"));
            functions.name = name.to_string();
            functions.is_global = false;

            if !arg.trim().is_empty() {
                let args = arg.split(",");
                for i in args {
                    let pts: Vec<&str> = i.split(":").collect();
                    if pts.len() != 2 {
                        return Err(format!(
                            "Error at line {}: Invalid argument declaration.\nCode:\n   => {}\nHint: Ensure that each argument is declared as 'name:type'.",
                            index as i32, ln
                        ));
                    }
                    let (name, t) = (pts[0].trim(), pts[1].trim());
                    if name.is_empty() {
                        return Err(format!(
                            "Error at line {}: Argument name cannot be empty.\nCode:\n   => {}\nHint: Provide a valid name for the argument.",
                            index as i32, ln
                        ));
                    }
                    match t {
                        "string" => functions.args.push(Args::Str(name.to_string())),
                        "int" => functions.args.push(Args::Int(name.to_string())),
                        "float" => functions.args.push(Args::Float(name.to_string())),
                        _ => {
                            return Err(format!(
                                "Error at line {}: Invalid argument type '{}'.\nCode:\n   => {}\nHint: Use 'string', 'int', or 'float' as argument types.",
                                index, t, ln
                            ));
                        }
                    }
                }
            }
            inf = true;
        } else if ln.starts_with("pub fn ") && ln.ends_with("{") {
            if inf {
                return Err(format!(
                    "Error at line {}: Nested function definitions are not allowed.\nCode:\n   => {}\nHint: Ensure that functions are properly closed before starting a new one.",
                    index, ln
                ));
            }
            let pts: Vec<&str> = ln[7..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "Error at line {}: Incorrect function declaration format.\nCode:\n   => {}\nHint: Ensure the function is declared as 'pub fn name(arg1:type, arg2:type){{'.",
                    index as i32, ln
                ));
            }
            let (name, mut arg) = (pts[0].trim(), pts[1].trim_end_matches("){"));
            functions.name = name.to_string();
            functions.is_global = true;
            if !arg.trim().is_empty() {
                let args = arg.split(",");
                for i in args {
                    let pts: Vec<&str> = i.split(":").collect();
                    if pts.len() != 2 {
                        return Err(format!(
                            "Error at line {}: Invalid argument declaration.\nCode:\n   => {}\nHint: Ensure that each argument is declared as 'name:type'.",
                            index as i32, ln
                        ));
                    }
                    let (name, t) = (pts[0].trim(), pts[1].trim());
                    if name.is_empty() {
                        return Err(format!(
                            "Error at line {}: Argument name cannot be empty.\nCode:\n   => {}\nHint: Provide a valid name for the argument.",
                            index as i32, ln
                        ));
                    }
                    match t {
                        "string" => functions.args.push(Args::Str(name.to_string())),
                        "int" => functions.args.push(Args::Int(name.to_string())),
                        "float" => functions.args.push(Args::Float(name.to_string())),
                        _ => {
                            return Err(format!(
                                "Error at line {}: Invalid argument type '{}'.\nCode:\n   => {}\nHint: Use 'string', 'int', or 'float' as argument types.",
                                index, t, ln
                            ));
                        }
                    }
                }
            }
            inf = true;
        } else if inf {
            if ln == "}" || ln.ends_with("}") {
                functions.code = fnbody.clone();
                inf = false;
            } else {
                let lv_clone = lv.clone();
                let ptkn = parse_single_line(ln.trim(), index, p_label, &mut lv, fnbody.clone());
                match ptkn {
                    Ok(tkn) => {
                        println!("tkn : {:?}", tkn);
                        match tkn {
                            Tokens::Var(v, n, _) => {
                                lv.push(fvars { v, n });
                            }
                            _ => {
                                fnbody.push(tkn.clone());
                                functions.code.push(tkn.clone());
                                fnbody.push(tkn);
                            }
                        }
                    }
                    Err(e) => match e.as_str() {
                        "|_EMP_|" => continue,
                        _ => return Err(e),
                    },
                }
            }
        } else if ln.is_empty() {
            return Err(format!(
                "Error at line {}: Unexpected empty line.\nCode:\n   => {}\nHint: Ensure that the code is properly formatted and not left blank where code is expected.",
                index, ln
            ));
        } else {
            return Err(format!(
                "Error at line {}: Unrecognized line format.\nCode:\n   => {}\nHint: Ensure that the line follows the expected function declaration or body format.",
                index, ln
            ));
        }
    }

    if inf {
        return Err("Error: File ended with an open function body.\nHint: Ensure that all opened functions are properly closed with '}'.".to_string());
    }
    //println!("function : {:?}", functions);
    functions.local_vars = lv;
    Ok(functions)
}
