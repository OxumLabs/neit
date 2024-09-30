use super::types::{Tokens, Vars};

pub fn evaluate_expression(expr: &str, vrs: &Vec<Tokens>) -> Result<f64, String> {
    let expr = expr.replace(" ", ""); // Remove spaces
    let mut tokens = tokenize(&expr)?;
    let result = parse_expression(&mut tokens, vrs)?; // Accept mutable reference but do not modify
    Ok(result)
}

fn tokenize(expr: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut pos = 0;

    while pos < expr.len() {
        let c = expr.chars().nth(pos).unwrap();
        pos += 1;

        if c.is_digit(10) || c == '.' {
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
                ' ' => {}
                _ => {
                    return Err(format!(
                        "Syntax Error: Invalid character '{}' at position {}",
                        c, pos
                    ))
                }
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

fn parse_expression(tokens: &mut Vec<String>, vrs: &Vec<Tokens>) -> Result<f64, String> {
    parse_add_sub(tokens, vrs) // Accepts tokens mutably but does not alter vrs
}

fn parse_add_sub(tokens: &mut Vec<String>, vrs: &Vec<Tokens>) -> Result<f64, String> {
    let mut value = parse_mul_div(tokens, vrs)?;

    while !tokens.is_empty() {
        if let Some(op) = tokens.get(0) {
            if op == "+" || op == "-" {
                let op = tokens.remove(0); // Mutable borrow happens here
                let rhs = parse_mul_div(tokens, vrs)?;
                value = if op == "+" { value + rhs } else { value - rhs };
            } else {
                break;
            }
        }
    }

    Ok(value)
}

fn parse_mul_div(tokens: &mut Vec<String>, vrs: &Vec<Tokens>) -> Result<f64, String> {
    let mut value = parse_exponentiation(tokens, vrs)?;

    while !tokens.is_empty() {
        if let Some(op) = tokens.get(0) {
            if op == "*" || op == "/" || op == "%" || op == "//" {
                let op = tokens.remove(0); // Mutable borrow happens here
                let rhs = parse_exponentiation(tokens, vrs)?;
                value = match op.as_str() {
                    "*" => value * rhs,
                    "/" => {
                        if rhs == 0.0 {
                            return Err("Math Error: Division by zero".to_string());
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

fn parse_exponentiation(tokens: &mut Vec<String>, vrs: &Vec<Tokens>) -> Result<f64, String> {
    let mut value = parse_primary(tokens, vrs)?;

    while !tokens.is_empty() {
        if let Some(op) = tokens.get(0) {
            if op == "**" {
                tokens.remove(0); // Mutable borrow happens here
                let rhs = parse_primary(tokens, vrs)?;
                value = value.powf(rhs);
            } else {
                break;
            }
        }
    }

    Ok(value)
}

fn parse_primary(tokens: &mut Vec<String>, vrs: &Vec<Tokens>) -> Result<f64, String> {
    if tokens.is_empty() {
        return Err("Syntax Error: Unexpected end of input".to_string());
    }

    let token = tokens.remove(0); // Remove token while parsing

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
        for i in vrs {
            if let Tokens::Var(v, n, _) = i {
                if token == *n {
                    return match v {
                        Vars::STR(_) => Err(format!("Type Error: Variable '{}' is a string and cannot be used in an arithmetic expression", token)),
                        Vars::F(f) => Ok(*f),
                        Vars::INT(i) => Ok(*i as f64),
                        Vars::EX(e) => evaluate_expression(&e, &mut vrs.clone()), // Pass mutable reference without cloning
                    };
                }
            }
        }
        Err(format!("Name Error: Undefined variable '{}'", token))
    }
}
