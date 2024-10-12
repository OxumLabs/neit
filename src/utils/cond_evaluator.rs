use super::types::{Tokens, Vars}; // Keep this if Vars is used, otherwise remove it.

pub fn eval_cond(cond: &str, vars: &[Tokens]) -> Result<String, String> {
    let mut curwrd = String::new(); // Collects the current word being parsed
    let mut result = String::new(); // Placeholder result
    let mut chars = cond.chars().enumerate().peekable(); // Peekable iterator to check next character
    let mut current_type: Option<String> = None; // Track the current type for consistency

    // Define the valid condition operators
    let operators = ["!=", "==", ">=", "=<", "<=", "=>", "="];

    // Iterate through characters
    while let Some((_i, c)) = chars.next() {
        let nxtc = chars.peek().map(|(_, next_c)| *next_c); // Peek next character

        match c {
            // If we find a space or an operator, we check the current word
            ' ' | '!' | '=' | '>' | '<' => {
                // Evaluate the current word before handling the operator
                if !curwrd.is_empty() {
                    if let Some(var_type) = check_word_type(&curwrd, vars) {
                        if let Some(ref cur_type) = current_type {
                            // Ensure type consistency or perform promotion
                            if !is_type_compatible(cur_type, &var_type) {
                                return Err(format!(
                                    "Type mismatch: '{}' is not compatible with '{}'",
                                    cur_type, var_type
                                ));
                            }
                        } else {
                            // Set the initial type for further checks
                            current_type = Some(var_type.clone());
                        }

                        // Append the valid word to the result
                        result.push_str(&curwrd);
                    } else {
                        return Err(format!("Invalid word: {}", curwrd));
                    }
                }
                curwrd.clear(); // Reset the current word

                // Handle operators (checking if it's a valid one)
                if let Some(op) = nxtc {
                    let mut operator = c.to_string();
                    operator.push(op);

                    if operators.contains(&operator.as_str()) {
                        chars.next(); // Consume the second part of the operator
                        result.push_str(&operator); // Add the operator to the result
                    } else {
                        result.push(c); // If it's just one part, add it directly
                    }
                }
            }
            _ => {
                // If it's a regular character (part of a word), append it to the current word
                curwrd.push(c);
            }
        }
    }

    // Check the last collected word (in case string doesn't end with a space or operator)
    if !curwrd.is_empty() {
        if let Some(var_type) = check_word_type(&curwrd, vars) {
            if let Some(ref cur_type) = current_type {
                if !is_type_compatible(cur_type, &var_type) {
                    return Err(format!(
                        "Type mismatch: '{}' is not compatible with '{}'",
                        cur_type, var_type
                    ));
                }
            }
            result.push_str(&curwrd);
        } else {
            return Err(format!("Invalid word: {}", curwrd));
        }
    }

    Ok(result) // Return the evaluated result (or error message)
}

// Check if the word is a number or a valid variable from `vars`, returning its type
fn check_word_type(word: &str, vars: &[Tokens]) -> Option<String> {
    if word.parse::<i32>().is_ok() {
        Some("int".to_string())
    } else if word.parse::<f64>().is_ok() {
        Some("float".to_string())
    } else {
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
}

// Check if the two types are compatible (supports int to float promotion)
fn is_type_compatible(current_type: &str, new_type: &str) -> bool {
    (current_type == new_type)
        || (current_type == "float" && new_type == "int")
        || (current_type == "int" && new_type == "float")
}
