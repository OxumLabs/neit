use super::{tokens::func::process_pub_func, types::Tokens};

#[allow(unused)]
pub fn gentoken(code: Vec<&str>) -> Result<Vec<Tokens>, String> {
    let mut index = 0;
    let mut tokens: Vec<Tokens> = Vec::new();
    for ln in code {
        index += 1;
        if ln.is_empty() {
            continue;
        } else if ln.trim().starts_with("pub fn") && ln.trim().ends_with("}") {
            let p_fn = process_pub_func(ln, index);
            match p_fn {
                Ok(func) => {
                    for tkn in &tokens {
                        match tkn {
                            Tokens::Func(f) => {
                                if f.name == func.name {
                                    return Err(format!(
                                        "Error at line {}: Function '{}' has already been declared at a previously",
                                        index, func.name
                                    ));
                                }
                            }
                        }
                    }
                    tokens.push(Tokens::Func(func));
                }
                Err(e) => return Err(e),
            }
        } else if ln.starts_with("fn ") {
            let p_fn = process_pub_func(ln, index);
            match p_fn {
                Ok(func) => {
                    for tkn in &tokens {
                        match tkn {
                            Tokens::Func(f) => {
                                if f.name == func.name {
                                    return Err(format!(
                                        "Error at line {}: Function '{}' has already been declared at a previously",
                                        index, func.name
                                    ));
                                }
                            }
                        }
                    }
                    tokens.push(Tokens::Func(func));
                }
                Err(e) => return Err(e),
            }
        }
        /* Add more */
        else {
            return Err(format!(
                "Error at line {} : Invalid Code?\n      => {}",
                index, ln
            ));
        }
    }
    Ok(tokens)
}
