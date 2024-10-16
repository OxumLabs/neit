use super::types::{Tokens, Vars};

pub fn eval_cond(cond: &str, vars: &[Tokens]) -> Result<String, String> {
    let mut curwrd = String::new();
    let mut result = String::new();
    let binding = cond.replace(" ", "");
    let mut chars = binding.chars().enumerate().peekable();
    let mut current_type: Option<String> = None;
    let _operators = ["!=", "==", ">=", "<=", ">", "<"];

    while let Some((i, c)) = chars.next() {
        let nxtc = chars.peek().map(|(_, next_c)| *next_c);

        match c {
            '!' | '=' | '>' | '<' => {
                if !curwrd.is_empty() {
                    if let Some(var_type) = check_word_type(&curwrd, vars) {
                        if let Some(ref cur_type) = current_type {
                            if !is_type_compatible(cur_type, &var_type) {
                                return Err(format!(
                                    "Type mismatch at index {}: expected type '{}', found type '{}'",
                                    i, cur_type, var_type
                                ));
                            }
                        } else {
                            current_type = Some(var_type.clone());
                        }
                        result.push_str(&curwrd);
                    } else {
                        return Err(format!("Invalid word at index {}: '{}'", i, curwrd));
                    }
                }
                curwrd.clear();

                if let Some(op) = nxtc {
                    let mut operator = c.to_string();
                    operator.push(op);

                    match operator.as_str() {
                        "!=" | "==" | ">=" | "<=" => {
                            chars.next();
                            result.push_str(&operator);
                        }
                        "=<" | "=>" | "=!" | "===" => {
                            return Err(format!(
                                "Invalid operator at index {}: '{}'. Did you mean '{}'?",
                                i,
                                operator,
                                correct_operator(&operator)
                            ));
                        }
                        "><" => {
                            return Err(format!(
                                "Invalid operator at index {}: '{}'. Did you mean '!='?",
                                i, operator
                            ));
                        }
                        "!!" => {
                            return Err(format!(
                                "Invalid operator at index {}: '{}'. Did you mean '!'?",
                                i, operator
                            ));
                        }
                        "<>" => {
                            return Err(format!(
                                "Invalid operator at index {}: '{}'. Did you mean '!='?",
                                i, operator
                            ));
                        }
                        "=" => {
                            return Err(format!(
                                "Invalid operator at index {}: single '=' is not allowed",
                                i
                            ));
                        }
                        "!" => {
                            return Err(format!(
                                "Invalid operator at index {}: '!' must be followed by '=' (!=)",
                                i
                            ));
                        }
                        _ => {
                            result.push(c);
                        }
                    }
                }
            }
            _ => {
                curwrd.push(c);
            }
        }
    }

    if !curwrd.is_empty() {
        if let Some(var_type) = check_word_type(&curwrd, vars) {
            if let Some(ref cur_type) = current_type {
                if !is_type_compatible(cur_type, &var_type) {
                    return Err(format!(
                        "Final type mismatch: expected type '{}', found type '{}' for '{}'",
                        cur_type, var_type, curwrd
                    ));
                }
            }
            result.push_str(&curwrd);
        } else {
            return Err(format!("Invalid word in final check: '{}'", curwrd));
        }
    }

    Ok(result)
}

fn check_word_type(word: &str, vars: &[Tokens]) -> Option<String> {
    if word.starts_with('"') && word.ends_with('"') {
        return Some("string".to_string());
    }

    if word.parse::<i32>().is_ok() {
        return Some("int".to_string());
    }

    if word.parse::<f64>().is_ok() {
        return Some("float".to_string());
    }

    vars.iter().find_map(|token| match token {
        Tokens::Var(var_type, var_name, _) if var_name == word => match var_type {
            Vars::STR(_) => Some("string".to_string()),
            Vars::INT(_) => Some("int".to_string()),
            Vars::F(_) => Some("float".to_string()),
            _ => None,
        },
        _ => None,
    })
}

fn is_type_compatible(current_type: &str, new_type: &str) -> bool {
    (current_type == new_type)
        || (current_type == "float" && new_type == "int")
        || (current_type == "int" && new_type == "float")
        || (current_type == "string" && new_type == "string")
}

fn correct_operator(invalid_op: &str) -> &str {
    match invalid_op {
        "=<" => "<=",
        "=>" => ">=",
        "=!" => "!=",
        "===" => "==",
        "==!" => "!=",
        "<>" => "!=",
        _ => "",
    }
}

pub fn ctoc(cond: &str, _vars: &[Tokens]) -> Result<String, String> {
    let mut r = String::new();
    let cond = cond.trim();
    let mut insmode = false;
    let mut aism = false;
    let ndw = ['=', '!', '<', '>'];
    let condd: Vec<char> = cond.chars().peekable().collect();

    let mut i = 0;
    while i < condd.len() {
        let c = condd[i];

        match c {
            '"' => {
                insmode = !insmode;

                if insmode {
                    if !aism {
                        r.push_str("strcmp(");
                        aism = true;
                    } else {
                        r.push(',');
                    }
                    r.push(c);
                } else {
                    r.push(c);
                    if i + 1 < condd.len() && !ndw.contains(&condd[i + 1]) {
                        r.push_str(",");
                    }
                }
            }
            _ if !ndw.contains(&c) => r.push(c),
            _ => {}
        }
        i += 1;
    }

    if aism {
        r.push_str(") == 0");
    }

    Ok(r)
}
