use crate::utils::{maths::evaluate_expression, types::Tokens};

pub fn process_print(num: &mut i32, text: &str, vars: &Vec<Tokens>) -> Tokens {
    //println!("text : {}", text);
    *num += 1;
    let mut result_text = String::new();
    let mut inside_string = false;
    let mut current_var = String::new();
    let mut is_var = false;
    let mut expression_mode = false;
    let mut open_brace_count = 0;

    for c in text.chars() {
        match c {
            '"' => {
                inside_string = !inside_string;
                if !inside_string && !current_var.is_empty() {
                    if let Ok(var_value) = evaluate_expression(&current_var[1..], vars) {
                        result_text.push_str(&var_value.to_string());
                        current_var.clear();
                    }
                }
            }
            '{' if inside_string && !is_var => {
                current_var.push(c);
                is_var = true;
                expression_mode = true;
                open_brace_count += 1;
            }
            '}' if is_var => {
                open_brace_count -= 1;

                if open_brace_count == 0 {
                    if let Ok(var_value) = evaluate_expression(&current_var[1..], vars) {
                        result_text.push_str(&var_value.to_string());
                        current_var.clear();
                        is_var = false;
                        expression_mode = false;
                    } else {
                        return Tokens::Print(
                            format!("Error: Invalid expression '{}'", current_var),
                            format!("p{}", num),
                        );
                    }
                }
            }
            _ => {
                if is_var && inside_string {
                    if expression_mode || c.is_alphanumeric() || c == '_' {
                        current_var.push(c);
                    } else {
                        result_text.push(c);
                        is_var = false;
                    }
                } else if inside_string {
                    result_text.push(c);
                }
            }
        }
    }

    if !current_var.is_empty() {
        if let Ok(var_value) = evaluate_expression(&current_var[1..], vars) {
            result_text.push_str(&var_value.to_string());
        }
    }

    //println!("resulted text : {}", result_text);
    Tokens::Print(result_text, format!("p{}", num))
}
