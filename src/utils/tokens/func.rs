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
                    "🚫 Whoopsie! Something's funky at line {}! 🤨\n\
                     💥 Invalid argument declaration detected! 😱\n\
                     Code:\n   => {}\n\
                     🔍 Hint: Remember, arguments should look like 'name:type'.\n\
                     🎨 Let's keep it tidy, okay? 😁",
                    index as i32, ln
                ));
            }
            let (name, t) = (pts[0], pts[1]);
            if name.is_empty() {
                return Err(format!(
                    "🚫 Uh-oh! I hit a snag at line {}! 😬\n\
                     🕳️ Looks like the argument name is missing... it's just *poof*, gone! 🤷‍♂️\n\
                     Code:\n   => {}\n\
                     🔍 Hint: Give your argument a proper name, it's feeling a bit left out! 😄",
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
                        "🚫 Yikes! Something's off at line {}! 😅\n\
                         🤨 The argument type '{}' is a bit too weird for me to handle!\n\
                         Code:\n   => {}\n\
                         🔍 Hint: Stick to 'string', 'int', or 'float'. My brain only speaks those languages! 😆",
                        index, t, ln
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
                    "🚫 Oops! I've got a bit of a mix-up at line {}! 😜\n\
                     🛠️ The function declaration is looking a little funky!\n\
                     Code:\n   => {}\n\
                     🔍 Hint: Make sure your function looks like this: 'pub fn name(arg1:type, arg2:type)'.\n\
                     🎯 It's gotta be spot-on or I get confused! 😅",
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
                    "🚫 Whoopsie! I've hit a snag at line {}! 🤭\n\
                     🛠️ Looks like your function declaration is a little off!\n\
                     Code:\n   => {}\n\
                     🔍 Hint: It should look like this: 'fn name(arg1:type, arg2:type)'.\n\
                     🎉 Let's get it sorted so I can do my thing! 😄",
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
                    "🚫 Uh-oh! I've run into a little trouble at line {}! 😬\n\
                     🏗️ It seems like you're trying to build a function inside another function, and that's a No-No!! 🚫\n\
                     Code:\n   => {}\n\
                     🔍 Hint: Close the first function before starting a new one—let's keep things neat and tidy! 😅",
                    index, ln
                ));
            }
            let pts: Vec<&str> = ln[3..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "🚫 Oopsie! I've stumbled upon a little hiccup at line {}! 🤭\n\
                     🛠️ It looks like your function declaration format is a bit off!\n\
                     Code:\n   => {}\n\
                     🔍 Hint: Make sure it follows this pattern: 'fn name(arg1:type){{'.\n\
                     🎉 I'm ready to help once it’s sorted out! 😄",
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
                    "🚫 Uh-oh! I've encountered a little snag at line {}! 😬\n\
                     🏗️ It looks like you’re trying to define a function inside another function, and that's a big no-no! 🚫\n\
                     Code:\n   => {}\n\
                     🔍 Hint: Close the previous function before starting a new one—let's keep things organized! 😅",
                    index, ln
                ));
            }
            let pts: Vec<&str> = ln[7..].split("(").collect();
            if pts.len() != 2 {
                return Err(format!(
                    "🚫 Error at line {}: Incorrect function declaration format.\n\
                     Code:\n   => {}\n\
                     🔍 Hint: Ensure the format is 'pub fn name(arg1:type){{'.",
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
                        _ => return Err(e),
                    },
                }
            }
        } else if ln.is_empty() {
            return Err(format!(
                "🚫 Oops! I’ve tripped over an unexpected empty line at line {}! 😅\n\
                 🕳️ Looks like there’s a little blank space here that’s throwing me off! 🤔\n\
                 Code:\n   => {}\n\
                 🔍 Hint: Let’s make sure that code lines aren’t left blank—keep it tidy for me! 🎉",
                index, ln
            ));
        } else {
            return Err(format!(
                "🚫 Whoops! I'm scratching my head at line {}! 🤔\n\
                 📜 It seems like the line format has me all confused! 😅\n\
                 Code:\n   => {}\n\
                 🔍 Hint: Double-check that it matches the expected function declaration or body format—let's keep it clear for both of us! 🎉",
                index, ln
            ));
        }
    }

    if inf {
        return Err(format!(
            "🚫 Oh no! The file ended, but I still see an open function body! 😱\n\
             🔍 Hint: Don't forget to close all opened functions with '}}' before you finish! 🛠️\n\
             Let's wrap it up so I can do my thing! 😊"
        ));
    }

    functions.local_vars = lv;
    Ok(functions)
}
