use super::types::{Tokens, Vars};

pub fn eval_cond(cond: &str, vars: &[Tokens]) -> Result<String, String> {
    let mut curwrd = String::new();
    let mut result = String::new();
    let binding = cond.replace(" ", "");

    let mut chars = binding.chars().enumerate().peekable();
    let mut current_type: Option<String> = None;
    let _operators = ["!=", "==", ">=", "<=", ">", "<"];
    let mut in_strcmp = false; // Flag to track if we're inside a strcmp operation

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

                        // Handle strcmp for string comparisons
                        if current_type == Some("string".to_string()) && in_strcmp {
                            result.push_str(&curwrd);
                            result.push_str(")");
                            in_strcmp = false;
                        } else {
                            result.push_str(&curwrd);
                        }
                    } else {
                        return Err(format!("Invalid word at index {}: '{}'", i, curwrd));
                    }
                }
                curwrd.clear();

                if let Some(op) = nxtc {
                    let mut operator = c.to_string();
                    operator.push(op);
                    match operator.as_str() {
                        "!=" | "==" => {
                            chars.next(); // Move past the operator
                            if let Some(ref cur_type) = current_type {
                                if cur_type == "string" {
                                    result.push_str("strcmp("); // Begin strcmp
                                    result.push_str(&curwrd); // Add the first operand
                                    result.push_str(","); // Prepare for the second operand
                                    in_strcmp = true; // Set flag indicating inside strcmp
                                    continue; // Continue to the next token
                                }
                            }
                            result.push_str(&operator);
                        }
                        ">=" | "<=" if current_type != Some("string".to_string()) => {
                            chars.next();
                            result.push_str(&operator);
                        }
                        ">" | "<" if current_type != Some("string".to_string()) => {
                            result.push(c);
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

            // Handle strcmp for final word
            if current_type == Some("string".to_string()) && in_strcmp {
                result.push_str(&curwrd);
                result.push_str(")");
            } else {
                result.push_str(&curwrd);
            }
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

    for token in vars {
        if let Tokens::Var(var_type, var_name, _) = token {
            if var_name == word {
                match var_type {
                    Vars::STR(_) => {
                        return Some("string".to_string());
                    }
                    Vars::INT(_) => {
                        return Some("int".to_string());
                    }
                    Vars::F(_) => {
                        return Some("float".to_string());
                    }
                    _ => {}
                }
            }
        }
    }

    None
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
