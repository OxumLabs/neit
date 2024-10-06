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
            if pts.len() != 2 {
                return Err(format!(
                    "✘ Yikes! Invalid argument declaration at line {}.\n\
                     → Hint: Arguments should be in the format 'name:type'.\n\
                     ⚙ [Code: {}]",
                    index as i32, ln
                ));
            }
            let (name, t) = (pts[0], pts[1]);
            if name.is_empty() {
                return Err(format!(
                    "✘ Yikes! Argument name is missing at line {}.\n\
                     → Hint: Provide a valid argument name.\n\
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
                        "✘ Yikes! Unrecognized argument type '{}' at line {}.\n\
                         → Hint: Supported types are 'string', 'int', or 'float'.\n\
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
        if ln.starts_with("pub fn ") && ln.ends_with("{}") {
            let pts: Vec<&str> = ln[7..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "✘ Yikes! Function declaration format is incorrect at line {}.\n\
                     → Hint: Ensure the format is 'pub fn name(arg1:type, arg2:type)'.\n\
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
                    "✘ Yikes! Function declaration format is incorrect at line {}.\n\
                     → Hint: Ensure the format is 'fn name(arg1:type, arg2:type)'.\n\
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
                    "✘ Yikes! Nested function definitions are not allowed at line {}.\n\
                     → Hint: Close the previous function before defining a new one.\n\
                     ⚙ [Code: {}]",
                    index, ln
                ));
            }
            let pts: Vec<&str> = ln[3..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "✘ Yikes! Function declaration format is incorrect at line {}.\n\
                     → Hint: Ensure the format is 'fn name(arg1:type){{'.\n\
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
                    "✘ Yikes! Nested function definitions are not allowed at line {}.\n\
                     → Hint: Close the previous function before defining a new one.\n\
                     ⚙ [Code: {}]",
                    index, ln
                ));
            }
            let pts: Vec<&str> = ln[7..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "✘ Yikes! Function declaration format is incorrect at line {}.\n\
                     → Hint: Ensure the format is 'pub fn name(arg1:type){{'.\n\
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
                                "✘ Yikes! Unexpected error encountered at line {}.\n\
                             → Hint: Review the syntax and try again.\n\
                             ⚙ [Error: {}]",
                                index, e
                            ))
                        }
                    },
                }
            }
        } else if ln.is_empty() {
            continue;
            return Err(format!(
                "✘ Yikes! Unexpected empty line encountered at line {}.\n\
                 → Hint: Remove empty lines to maintain clarity.\n\
                 ⚙ [Code: {}]",
                index, ln
            ));
        } else {
            return Err(format!(
                "✘ Yikes! Unrecognized line format at line {}.\n\
                 → Hint: Check the line for proper function declaration or body syntax.\n\
                 ⚙ [Code: {}]",
                index, ln
            ));
        }
    }

    if inf {
        return Err(format!(
            "✘ Yikes! Open function body without closure detected.\n\
             → Hint: Ensure all opened functions are properly closed before ending the file."
        ));
    }

    functions.local_vars = lv;
    Ok(functions)
}
