pub fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let expr = expr.replace(" ", ""); // Remove spaces for simplicity
    let mut tokens = tokenize(&expr)?;
    let result = parse_expression(&mut tokens)?;
    Ok(result)
}

fn tokenize(expr: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for c in expr.chars() {
        if c.is_digit(10) || c == '.' {
            current.push(c);
        } else if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            tokens.push(c.to_string());
        } else {
            return Err(format!("Invalid character in expression: {}", c));
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

fn parse_expression(tokens: &mut Vec<String>) -> Result<f64, String> {
    parse_add_sub(tokens)
}

fn parse_add_sub(tokens: &mut Vec<String>) -> Result<f64, String> {
    let mut value = parse_mul_div(tokens)?;

    while let Some(op) = &tokens.clone().get(0) {
        if *op == "+" || *op == "-" {
            tokens.remove(0);
            let rhs = parse_mul_div(tokens)?;
            value = if *op == "+" { value + rhs } else { value - rhs };
        } else {
            break;
        }
    }

    Ok(value)
}

fn parse_mul_div(tokens: &mut Vec<String>) -> Result<f64, String> {
    let mut value = parse_primary(tokens)?;

    while let Some(op) = tokens.clone().get(0) {
        if op == "*" || op == "/" {
            tokens.remove(0);
            let rhs = parse_primary(tokens)?;
            value = if op == "*" { value * rhs } else { value / rhs };
        } else {
            break;
        }
    }

    Ok(value)
}

fn parse_primary(tokens: &mut Vec<String>) -> Result<f64, String> {
    if tokens.is_empty() {
        return Err("Unexpected end of input".to_string());
    }

    let token = tokens.remove(0);

    if token == "(" {
        let value = parse_expression(tokens)?;
        if tokens.remove(0) != ")" {
            return Err("Mismatched parentheses".to_string());
        }
        Ok(value)
    } else if token.chars().all(|c| c.is_digit(10) || c == '.') {
        token.parse::<f64>().map_err(|e| e.to_string())
    } else {
        Err(format!("Unexpected token: {}", token))
    }
}
