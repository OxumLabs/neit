use super::types::{Tokens, Vars};

pub fn evaluate_expression(expr: &str, vrs: &Vec<Tokens>) -> Result<f64, String> {
    let expr = expr.replace(" ", "");
    let mut tokens = tokenize(&expr)?;
    let result = parse_expression(&mut tokens, vrs)?;
    Ok(result)
}

fn tokenize(expr: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut pos = 0; // Track position in the expression

    while pos < expr.len() {
        let c = expr.chars().nth(pos).unwrap();
        pos += 1;

        if c.is_digit(10) || c == '.' {
            current.push(c);
        } else if c.is_alphabetic() {
            current.push(c);
        } else if c == '+' || c == '-' || c == '(' || c == ')' {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            tokens.push(c.to_string());
        } else if c == '*' {
            if pos < expr.len() && expr.chars().nth(pos) == Some('*') {
                pos += 1; // Skip the next '*' character
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push("**".to_string());
                continue;
            } else {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push("*".to_string());
            }
        } else if c == '/' {
            if pos < expr.len() && expr.chars().nth(pos) == Some('/') {
                pos += 1; // Skip the next '/' character
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push("//".to_string());
                continue;
            } else {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push("/".to_string());
            }
        } else if c == '%' {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            tokens.push("%".to_string());
        } else if c == ' ' {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else {
            return Err(format!(
                "Syntax Error: Invalid character '{}' at position {}",
                c, pos
            ));
        }
    }

    // Final check for any remaining tokens
    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

fn parse_expression(tokens: &mut Vec<String>, vrs: &Vec<Tokens>) -> Result<f64, String> {
    parse_add_sub(tokens, vrs)
}

fn parse_add_sub(tokens: &mut Vec<String>, vrs: &Vec<Tokens>) -> Result<f64, String> {
    let mut value = parse_mul_div(tokens, vrs)?;

    while let Some(op) = &tokens.clone().get(0) {
        if *op == "+" || *op == "-" {
            tokens.remove(0);
            let rhs = parse_mul_div(tokens, vrs)?;
            value = if *op == "+" { value + rhs } else { value - rhs };
        } else {
            break;
        }
    }

    Ok(value)
}

fn parse_mul_div(tokens: &mut Vec<String>, vrs: &Vec<Tokens>) -> Result<f64, String> {
    let mut value = parse_exponentiation(tokens, vrs)?;

    while let Some(op) = tokens.clone().get(0) {
        if op == "*" || op == "/" || op == "%" || op == "//" {
            tokens.remove(0);
            let rhs = parse_exponentiation(tokens, vrs)?;
            value = match op.as_str() {
                "*" => value * rhs,
                "/" => value / rhs,
                "%" => value % rhs,
                "//" => {
                    // Floor division logic
                    let result = value / rhs;
                    let floor_value = result.floor();
                    let decimal_part = result - floor_value;

                    // Calculate the final value based on the decimal part
                    if decimal_part < 0.5 {
                        floor_value
                    } else {
                        floor_value + 1.0
                    }
                }
                _ => value, // Should not reach here
            };
        } else {
            break;
        }
    }

    Ok(value)
}

fn parse_exponentiation(tokens: &mut Vec<String>, vrs: &Vec<Tokens>) -> Result<f64, String> {
    let mut value = parse_primary(tokens, vrs)?;

    while let Some(op) = tokens.clone().get(0) {
        if op == "**" {
            tokens.remove(0);
            let rhs = parse_primary(tokens, vrs)?;
            value = value.powf(rhs);
        } else {
            break;
        }
    }

    Ok(value)
}

fn parse_primary(tokens: &mut Vec<String>, vrs: &Vec<Tokens>) -> Result<f64, String> {
    if tokens.is_empty() {
        return Err("Syntax Error: Unexpected end of input".to_string());
    }

    let token = tokens.remove(0);

    if token == "(" {
        let value = parse_expression(tokens, vrs)?;
        if tokens.is_empty() || tokens.remove(0) != ")" {
            return Err("Syntax Error: Mismatched parentheses".to_string());
        }
        Ok(value)
    } else if token.chars().all(|c| c.is_digit(10) || c == '.') {
        token.parse::<f64>().map_err(|e| {
            format!(
                "Conversion Error: Failed to parse number '{}' - {}",
                token, e
            )
        })
    } else {
        // Handle variables from `vrs`
        for i in vrs {
            match i {
                Tokens::Var(v, n, _) => {
                    if token == *n {
                        return match v {
                            Vars::STR(_) => {
                                Err(format!("Type Error: Variable '{}' is a string and cannot be used in an arithmetic expression", token))
                            }
                            Vars::F(f) => Ok(*f),          // Float variable
                            Vars::INT(i) => Ok(*i as f64), // Integer variable
                            Vars::EX(e) => {
                                let result = evaluate_expression(&e, vrs);
                                match result {
                                    Ok(val) => Ok(val),
                                    Err(err) => Err(format!("Evaluation Error in expression for '{}': {}", token, err)),
                                }
                            }
                        };
                    }
                }
                _ => {}
            }
        }
        Err(format!("Name Error: Undefined variable '{}'", token))
    }
}
