use super::types::{Tokens, Vars};
use std::process::exit;

pub fn pp(num: &mut i32, text: &str, vars: &Vec<Tokens>) -> Tokens {
    *num += 1;
    let mut result_text = String::new();
    let mut inside_string = false;
    let mut current_var = String::new();
    let mut is_var = false;
    let mut open_brace_count = 0;

    for c in text.chars() {
        match c {
            '"' => {
                inside_string = !inside_string;
                if !inside_string && !current_var.is_empty() {
                    handle_current_var(&mut result_text, &current_var, vars);
                    current_var.clear();
                }
            }
            '{' if inside_string && !is_var => {
                is_var = true;
                open_brace_count += 1;
            }
            '}' if is_var => {
                open_brace_count -= 1;
                if open_brace_count == 0 {
                    handle_current_var(&mut result_text, &current_var, vars);
                    current_var.clear();
                    is_var = false;
                }
            }
            _ => {
                if is_var && inside_string {
                    if c.is_alphanumeric() || c == '_' {
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
        handle_current_var(&mut result_text, &current_var, vars);
    }
    Tokens::Print(result_text, format!("p{}", num))
}

fn handle_current_var(result_text: &mut String, current_var: &str, vars: &Vec<Tokens>) {
    let mut var_found = false;

    for v in vars {
        if let Tokens::Var(v_type, var_name, _) = v {
            if current_var == var_name {
                var_found = true;
                match v_type {
                    Vars::STR(_) => {
                        result_text.push_str(&format!("|{}~s|", var_name));
                        return; // Exit early if we only need to process the string variable
                    }
                    Vars::INT(_) => {
                        result_text.push_str(&format!("|{}~d|", var_name));
                        return;
                    }
                    Vars::F(_) => {
                        result_text.push_str(&format!("|{}~f|", var_name));
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

    // If current_var is not a known variable, evaluate it as an expression
    if !var_found {
        match evaluate_expression(current_var, vars) {
            Ok(value) => {
                result_text.push_str(&value.to_string());
            }
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }
}

pub fn evaluate_expression(expr: &str, vars: &Vec<Tokens>) -> Result<f64, String> {
    let expr = expr.replace(" ", ""); // Remove spaces
    let mut tokens = tokenize(&expr)?;
    let result = parse_expression(&mut tokens, vars)?;
    Ok(result)
}

fn tokenize(expr: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut pos = 0;

    while pos < expr.len() {
        let c = expr.chars().nth(pos).unwrap();
        pos += 1;

        if c.is_ascii_digit() || c == '.' {
            current.push(c);
        } else if c.is_alphabetic() {
            current.push(c);
        } else {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            match c {
                '+' | '-' | '(' | ')' => tokens.push(c.to_string()),
                '*' => {
                    if pos < expr.len() && expr.chars().nth(pos) == Some('*') {
                        pos += 1;
                        tokens.push("**".to_string());
                    } else {
                        tokens.push("*".to_string());
                    }
                }
                '/' => {
                    if pos < expr.len() && expr.chars().nth(pos) == Some('/') {
                        pos += 1;
                        tokens.push("//".to_string());
                    } else {
                        tokens.push("/".to_string());
                    }
                }
                '%' => tokens.push("%".to_string()),
                _ => {
                    return Err(format!(
                        "✘ Uh-oh! Syntax Error: I spotted an invalid character '{}' at position {}—it doesn’t belong here!\n\
                        → Reason: Let’s keep our code tidy and make sure only the right characters are hanging out!\n\
                        →→ Hint: Remember, every character has its place—let’s find it!",
                        c, pos
                    ));
                }
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

fn parse_expression(tokens: &mut Vec<String>, vars: &Vec<Tokens>) -> Result<f64, String> {
    parse_add_sub(tokens, vars) // Accepts tokens mutably but does not alter vars
}

fn parse_add_sub(tokens: &mut Vec<String>, vars: &Vec<Tokens>) -> Result<f64, String> {
    let mut value = parse_mul_div(tokens, vars)?;

    while !tokens.is_empty() {
        if let Some(op) = tokens.first() {
            if op == "+" || op == "-" {
                let op = tokens.remove(0); // Mutable borrow happens here
                let rhs = parse_mul_div(tokens, vars)?;
                value = if op == "+" { value + rhs } else { value - rhs };
            } else {
                break;
            }
        }
    }

    Ok(value)
}

fn parse_mul_div(tokens: &mut Vec<String>, vars: &Vec<Tokens>) -> Result<f64, String> {
    let mut value = parse_exponentiation(tokens, vars)?;

    while !tokens.is_empty() {
        if let Some(op) = tokens.first() {
            if op == "*" || op == "/" || op == "%" || op == "//" {
                let op = tokens.remove(0); // Mutable borrow happens here
                let rhs = parse_exponentiation(tokens, vars)?;
                value = match op.as_str() {
                    "*" => value * rhs,
                    "/" => {
                        if rhs == 0.0 {
                            return Err("✘ Math Error: Whoa there! You tried to divide by zero—yikes!\n\
                            → Reason: Remember, dividing by zero is like trying to find a unicorn in a haystack—it's just not gonna happen!"
                            .to_string());
                        }
                        value / rhs
                    }
                    "%" => value % rhs,
                    "//" => {
                        let result = value / rhs;
                        let fractional_part = result - result.floor();
                        if fractional_part >= 0.5 {
                            (result.floor() as i64 + 1) as f64
                        } else {
                            result.floor()
                        }
                    }
                    _ => unreachable!(),
                };
            } else {
                break;
            }
        }
    }

    Ok(value)
}

fn parse_exponentiation(tokens: &mut Vec<String>, vars: &Vec<Tokens>) -> Result<f64, String> {
    let mut value = parse_primary(tokens, vars)?;

    while !tokens.is_empty() {
        if let Some(op) = tokens.first() {
            if op == "**" {
                tokens.remove(0); // Mutable borrow happens here
                let rhs = parse_primary(tokens, vars)?;
                value = value.powf(rhs);
            } else {
                break;
            }
        }
    }

    Ok(value)
}

fn parse_primary(tokens: &mut Vec<String>, vars: &Vec<Tokens>) -> Result<f64, String> {
    if tokens.is_empty() {
        return Err("✘ Uh-oh! Syntax Error: Unexpected end of input! It’s like a cliffhanger in a movie—where’s the rest of the story?\n\
        → Reason: Don’t leave me hanging! Make sure to wrap things up properly to keep the code flowing smoothly!".to_string());
    }

    let token = tokens.remove(0); // Remove token while parsing

    if token == "(" {
        let value = parse_expression(tokens, vars)?;
        if tokens.is_empty() || tokens.remove(0) != ")" {
            return Err("✘ Oopsie! Syntax Error: Mismatched parentheses! It’s like a dance party where the pairs just don’t match up!\n\
            → Reason: Make sure every opening parenthesis has a buddy to close it—let’s keep those dances in sync!".to_string());
        }
        Ok(value)
    } else if token.chars().all(|c| c.is_ascii_digit() || c == '.') {
        // Handle numeric literals directly
        token.parse::<f64>().map_err(|e| {
            format!(
                "✘ Conversion Error: I couldn’t parse the number '{}'—it’s playing hard to get!\n\
                → Reason: {}. Let’s make sure it’s in tip-top shape to be converted!",
                token, e
            )
        })
    } else {
        for i in vars {
            if let Tokens::Var(v, n, _) = i {
                if token == *n {
                    return match v {
                        Vars::STR(_) => Ok(0.0), // You can change this to return a special value for strings
                        Vars::F(f) => Ok(*f),
                        Vars::INT(i) => Ok(*i as f64),
                        Vars::EX(e) => evaluate_expression(e, &mut vars.clone()), // Pass mutable reference without cloning
                    };
                }
            }
        }
        return Err(format!(
            "✘ Oopsie! It seems like I tripped over an undefined variable: '{}'.\n\
            → Reason: Make sure this little guy is declared somewhere before it's used in your code—let's not leave it hanging!",
            token
        ));
    }
}
