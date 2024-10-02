use std::process::exit;

use crate::utils::{
    // maths::evaluate_expression, // Not needed anymore since we're not evaluating expressions here.
    types::{Tokens, Vars},
};

pub fn process_print(num: &mut i32, text: &str, vars: &Vec<Tokens>) -> Tokens {
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
                    result_text.push_str(&current_var); // Add any leftover current_var.
                    current_var.clear();
                }
            }
            '{' if inside_string && !is_var => {
                is_var = true;
                expression_mode = true;
                open_brace_count += 1;
            }
            '}' if is_var => {
                open_brace_count -= 1;

                if open_brace_count == 0 {
                    let mut var_found = false;
                    for v in vars.clone() {
                        if let Tokens::Var(v, n, _) = v {
                            if current_var == n {
                                var_found = true;
                                // Generate the custom notation based on the variable type
                                match v {
                                    Vars::STR(_) => result_text.push_str(&format!("|{}~s|", n)),
                                    Vars::INT(_) => result_text.push_str(&format!("|{}~d|", n)),
                                    Vars::F(_) => result_text.push_str(&format!("|{}~f|", n)),
                                    _ => {}
                                }
                                current_var.clear();
                                is_var = false;
                                expression_mode = false;
                                break;
                            }
                        }
                    }

                    // If the variable was not found, return an error
                    if !var_found {
                        eprintln!("Error: Unknown variable '{}'", current_var,);
                        exit(1);
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

    // If there is any remaining content in current_var
    if !current_var.is_empty() {
        result_text.push_str(&current_var);
    }
    Tokens::Print(result_text, format!("p{}", num))
}

pub fn p_to_c(text: &str, _vars: &Vec<Tokens>) -> String {
    let mut c_code = String::new();
    c_code.push('\"');
    let mut collected_vars = Vec::new();
    let mut inside_var = false;
    let mut var_name = String::new();
    let mut literal_text = String::new();

    for c in text.chars() {
        if inside_var {
            if c == '|' {
                inside_var = false;
                // Split on '~' to get variable name and format
                let parts: Vec<&str> = var_name.split('~').collect();
                if parts.len() == 2 {
                    let var = parts[0];
                    let fmt = parts[1];

                    // Add the literal text to c_code and clear it
                    if !literal_text.is_empty() {
                        c_code.push_str(&literal_text);
                        literal_text.clear();
                    }

                    // Add the appropriate format specifier to the c_code
                    match fmt {
                        "s" => c_code.push_str("%s"),
                        "d" => c_code.push_str("%d"),
                        "f" => c_code.push_str("%f"),
                        _ => {}
                    }

                    // Collect the variable name for the argument list
                    collected_vars.push(var.to_string());
                }
                var_name.clear();
            } else {
                var_name.push(c);
            }
        } else if c == '|' {
            inside_var = true;
        } else {
            literal_text.push(c); // Collect literal text until we hit a variable
        }
    }

    // Add any remaining literal text after processing
    if !literal_text.is_empty() {
        c_code.push_str(&literal_text);
    }

    // Now append all the collected variables to the printf statement
    c_code.push('\"');
    if !collected_vars.is_empty() {
        c_code.push_str(", ");
        c_code.push_str(&collected_vars.join(", "));
    }

    c_code
}
