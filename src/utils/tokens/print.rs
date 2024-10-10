use std::process::exit;

use crate::{
    utils::{
        maths::evaluate_expression,
        types::{Tokens, Vars},
    },
    UCMF, UCMI,
};

pub fn process_print(num: &mut i32, text: &str, vars: &Vec<Tokens>) -> Tokens {
    // println!("text : {} |", text);
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
                // Ensure we only push formatted text when the string is closed
                if !inside_string && !current_var.is_empty() {
                    result_text.push_str(&current_var);
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

                    // Check if it's a valid variable or an expression
                    if !current_var.is_empty() {
                        for v in vars.iter() {
                            if let Tokens::Var(v_type, n, _) = v {
                                if current_var == *n {
                                    var_found = true;
                                    // Add format specifier for variables
                                    match v_type {
                                        Vars::STR(_) => result_text.push_str(&format!("|{}~s|", n)),
                                        Vars::INT(_) => result_text.push_str(&format!("|{}~d|", n)),
                                        Vars::F(_) => result_text.push_str(&format!("|{}~f|", n)),
                                        _ => {}
                                    }
                                    break;
                                }
                            }
                        }

                        // Handle case where variable was not found, treating it as an expression
                        if !var_found {
                            let value = evaluate_expression(&current_var, vars);
                            match value {
                                Ok(v) => {
                                    if v.to_string().parse::<i128>().is_ok() {
                                        result_text.push_str(&format!("|{}~d|", current_var));
                                    } else if v.to_string().parse::<f64>().is_ok() {
                                        result_text.push_str(&format!("|{}~f|", current_var));
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "✘ Error: Expression Evaluation Failed\n\
                                        Expression: '{}'\n\
                                        ➔ Reason: {}\n\
                                        Steps to troubleshoot:\n\
                                        1. Review the syntax for errors or typos.\n\
                                        2. Ensure all variables are defined and accessible.\n\
                                        3. Check for logical issues affecting evaluation.\n\
                                        → Please check the expression and try again.",
                                        current_var, e
                                    );

                                    exit(1);
                                }
                            }
                        }
                        // Clear current_var after processing
                        current_var.clear();
                        is_var = false;
                        expression_mode = false;
                    }
                }
            }
            _ => {
                if is_var && inside_string {
                    if expression_mode || c.is_alphanumeric() || c == '_' || c == ' ' {
                        current_var.push(c);
                    } else {
                        // If we encounter something that isn't part of a variable, stop processing the current variable
                        is_var = false;
                    }
                } else if inside_string {
                    result_text.push(c); // Collect literal text
                }
            }
        }
    }

    // If there is any remaining content in current_var
    if !current_var.is_empty() {
        result_text.push_str(&current_var); // This may happen at the end of the string
    }
    //println!("result text : {} |", result_text);
    Tokens::Print(result_text, format!("p{}", num))
}

pub fn p_to_c(text: &str, _vars: &Vec<Tokens>) -> String {
    let mut c_code = String::new();
    c_code.push_str("\""); // Start the printf statement with printf
    let mut collected_vars = Vec::new();
    let mut inside_var = false;
    let mut var_name = String::new();
    let mut literal_text = String::new();

    // Helper function to build nested calls
    fn build_nested_calls(parts: Vec<&str>) -> String {
        if parts.is_empty() {
            return String::new();
        }

        let first_value = parts[0].trim();
        let mut result = String::new();

        // Check if first_value is valid for parsing
        if let Ok(_) = first_value.parse::<f64>() {
            result.push_str(&format!("fdf({}", first_value));
            unsafe { UCMF = true };
        } else {
            result.push_str(&format!("fdi({}", first_value));
            unsafe { UCMI = true };
        }

        // Recursively build nested calls for the remaining parts
        for (i, &part) in parts[1..].iter().enumerate() {
            // Check if this is the last part
            if i == parts.len() - 2 {
                result.push_str(&format!(", {}", part.trim()));
                result.push(')'); // Close the initial fdf/fdi call
                break; // Exit after adding the last part
            } else {
                // Create a nested call for this part
                let nested_call = build_nested_calls(vec![part.trim()]);
                result.push_str(&format!(", {}", nested_call));
            }
        }
        result
    }

    // Iterate over each character in the input text
    for c in text.chars() {
        if inside_var {
            if c == '|' {
                inside_var = false; // End of variable
                let parts: Vec<&str> = var_name.split('~').collect();
                if parts.len() == 2 {
                    let var = parts[0];
                    let fmt = parts[1];

                    // Collect the variable name for the argument list
                    collected_vars.push(var.to_string());

                    // Add the appropriate format specifier to the literal text
                    match fmt {
                        "s" => literal_text.push_str("%s"),
                        "d" => literal_text.push_str("%d"),
                        "f" => literal_text.push_str("%f"),
                        _ => {}
                    }
                }
                var_name.clear(); // Clear variable name for next usage
            } else {
                var_name.push(c); // Accumulate variable name
            }
        } else if c == '|' {
            inside_var = true; // Start processing variable
        } else if c == '{' {
            // Start of an expression
            if !literal_text.is_empty() {
                c_code.push_str(&literal_text); // Push any accumulated literal text
                literal_text.clear(); // Clear literal text for next use
            }
            inside_var = true; // Next characters will be treated as variable or expression
        } else if c == '}' {
            inside_var = false; // End of an expression
            if !var_name.is_empty() {
                let expression = var_name.clone();
                if expression.contains("//") {
                    // Split on "//" for nested function calls
                    let pts: Vec<&str> = expression.split("//").collect();
                    let nested_call = build_nested_calls(pts);
                    literal_text.push_str(&nested_call);
                } else {
                    // Existing logic for handling single variables
                    let mut var_found = false;
                    for v in _vars.iter() {
                        if let Tokens::Var(v_type, n, _) = v {
                            if *n == expression {
                                var_found = true;
                                match v_type {
                                    Vars::STR(_) => {
                                        literal_text.push_str(&format!("|{}~s|", n));
                                    }
                                    Vars::INT(_) => {
                                        literal_text.push_str(&format!("|{}~d|", n));
                                    }
                                    Vars::F(_) => {
                                        literal_text.push_str(&format!("|{}~f|", n));
                                    }
                                    _ => {}
                                }
                                break;
                            }
                        }
                    }
                    // Handle case where variable was not found
                    if !var_found {
                        eprintln!(
                            "✘ Error: Variable Not Found\n\
                            Cannot find the variable '{}'.\n\
                            ➔ Ensure it's defined correctly and in scope.\n\
                            ➔ Check for typos in the variable name.\n\
                            ➔ If defined in another block or function, ensure accessibility.\n\
                            → Please review the declaration and try again.",
                            expression
                        );

                        exit(1);
                    }
                }
                var_name.clear(); // Clear variable name for next usage
            }
        } else {
            literal_text.push(c); // Collect literal text including spaces
        }
    }

    // After processing all characters, append any remaining literal text
    if !literal_text.is_empty() {
        c_code.push_str(&literal_text);
    }

    // Process collected variables for formatting
    // Process collected variables for formatting
    for cv in collected_vars.iter_mut() {
        if cv.contains("//") {
            let pts: Vec<&str> = cv.split("//").collect();
            let nested_call = build_nested_calls(pts.clone());
            let closing_parentheses_count = pts.len() - 1;
            *cv = format!(
                "{}{}",
                nested_call,
                ")".repeat(closing_parentheses_count - 1)
            );
        }
    }

    // Append all the collected variables to the printf statement
    if !collected_vars.is_empty() {
        c_code.push_str("\", ");
        c_code.push_str(&collected_vars.join(", ")); // Join collected variables with commas
    } else {
        c_code.push_str("\"");
    }

    c_code.push_str(""); // Close the printf statement
    c_code
}
