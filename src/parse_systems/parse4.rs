use super::{parse5::parse5, AST};
use crate::{err_system::err_types::ErrTypes, parse_systems::Variables, tok_system::tokens::Token};

#[inline(always)]
pub fn parse4(
    token: &Token,
    token_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    ast: &mut Vec<AST>,
    _code: &String,
    collected_vars: &mut Vec<(String, &'static str)>,
    collected_errors: &mut Vec<ErrTypes>,
    line: &mut i32,
) {
    let var_name: String = match token {
        Token::Iden(name) => {
            
            if name == "const" || name == "global" {
                
                parse5(
                    token,
                    token_iter,
                    ast,
                    _code,
                    collected_vars,
                    collected_errors,
                    line,
                );
    
                
                return;
            } else {
                
                name.clone()
            }
        }
        _ => {
            return;
        }
    };
    
    // Check if variable exists and is not const.
    let var_info_option = collected_vars
        .iter()
        .find(|(name, _)| name == &var_name)
        .cloned();
    if var_info_option.is_none() {
        collected_errors.push(ErrTypes::VarNotFound(*line));
        return;
    }
    let (_, var_type) = var_info_option.unwrap();
    if var_type.contains("const;") {
        collected_errors.push(ErrTypes::VarISConst(*line));
        return;
    }

    // Skip spaces.
    while matches!(token_iter.peek(), Some(Token::Space)) {
        token_iter.next();
    }

    // Check for assignment or compound operator.
    let mut compound_operator: Option<char> = None;
    if let Some(op_token) = token_iter.next() {
        match op_token {
            Token::EqSign => {}
            Token::ADDOP | Token::SUBOP | Token::MULTIOP | Token::DIVOP => {
                compound_operator = match op_token {
                    Token::ADDOP => Some('+'),
                    Token::SUBOP => Some('-'),
                    Token::MULTIOP => Some('*'),
                    Token::DIVOP => Some('/'),
                    _ => None,
                };
                while matches!(token_iter.peek(), Some(Token::Space)) {
                    token_iter.next();
                }
                if let Some(Token::EqSign) = token_iter.next() {
                } else {
                    collected_errors.push(ErrTypes::MissingOperator(*line));
                    return;
                }
            }
            _ => {
                collected_errors.push(ErrTypes::UnexpectedToken(*line));
                return;
            }
        }
    } else {
        collected_errors.push(ErrTypes::MissingOperator(*line));
        return;
    }
    while matches!(token_iter.peek(), Some(Token::Space)) {
        token_iter.next();
    }

    // Build the raw expression.
    let mut raw_value = String::new();
    while let Some(tok) = token_iter.peek() {
        match tok {
            Token::EOL | Token::EOF => {*line += 1;break},
            Token::Space => {
                token_iter.next();
            }
            Token::Iden(val) => {
                raw_value.push_str(val);
                token_iter.next();
            }
            Token::ADDOP => {
                raw_value.push('+');
                token_iter.next();
            }
            Token::SUBOP => {
                raw_value.push('-');
                token_iter.next();
            }
            Token::MULTIOP => {
                raw_value.push('*');
                token_iter.next();
            }
            Token::DIVOP => {
                raw_value.push('/');
                token_iter.next();
            }
            _ => {
                token_iter.next();
            }
        }
    }
    if raw_value.trim().is_empty() {
        collected_errors.push(ErrTypes::MissingValue(*line));
        return;
    }

    // Process each operand and rebuild expression.
    let mut final_expr = {
        let mut result = String::new();
        let mut current_operand = String::new();
        for c in raw_value.chars() {
            if "+-*/".contains(c) {
                if !current_operand.is_empty() {
                    if !validate_operand(
                        &current_operand,
                        &var_name,
                        collected_vars,
                        collected_errors,
                        *line,
                    ) {
                        return;
                    }
                    result.push_str(&format_operand(&current_operand));
                    current_operand.clear();
                }
                result.push(c);
            } else {
                current_operand.push(c);
            }
        }
        if !current_operand.is_empty() {
            if !validate_operand(
                &current_operand,
                &var_name,
                collected_vars,
                collected_errors,
                *line,
            ) {
                return;
            }
            result.push_str(&format_operand(&current_operand));
        }
        result
    };

    // Prepend compound assignment operator if any.
    if let Some(op) = compound_operator {
        final_expr = format!("{}{}{}", var_name, op, final_expr);
    }

    // If the expression is arithmetic, treat it as math.
    if final_expr.contains('+')
        || final_expr.contains('-')
        || final_expr.contains('*')
        || final_expr.contains('/')
    {
        let new_var = Variables::MATH(var_name.clone(), final_expr.clone());
        ast.push(AST::VarAssign(new_var));
        return;
    }

    // Create variable based on type.
    let new_var: Variables = if final_expr.starts_with('\"') && final_expr.ends_with('\"') {
        let processed = final_expr[1..final_expr.len() - 1].to_string();
        if var_type == "str" {
            Variables::Str(Box::leak(var_name.clone().into_boxed_str()), processed)
        } else {
            collected_errors.push(ErrTypes::TypeMismatch(*line));
            return;
        }
    } else if final_expr.starts_with('\'') && final_expr.ends_with('\'') {
        let processed = final_expr[1..final_expr.len() - 1].to_string();
        if processed.chars().count() != 1 {
            collected_errors.push(ErrTypes::CharVarLen(*line));
            return;
        }
        if var_type == "ch" {
            Variables::Char(
                Box::leak(var_name.clone().into_boxed_str()),
                processed.chars().next().unwrap(),
            )
        } else {
            collected_errors.push(ErrTypes::TypeMismatch(*line));
            return;
        }
    } else {
        let mut processed_value = final_expr.clone();
        let mut forced_type = None;
        if processed_value.ends_with(')') {
            if let Some(start) = processed_value.rfind('(') {
                let potential = processed_value[start + 1..processed_value.len() - 1]
                    .trim()
                    .to_string();
                if potential == "i8"
                    || potential == "i16"
                    || potential == "i32"
                    || potential == "i64"
                    || potential == "f32"
                    || potential == "f64"
                {
                    forced_type = Some(potential);
                    let trimmed_value = processed_value[..start].trim().to_string();
                    if trimmed_value.is_empty() {
                        collected_errors.push(ErrTypes::MissingValue(*line));
                        return;
                    }
                    processed_value = trimmed_value;
                }
            }
        }
        if let Some(ty) = forced_type {
            match ty.as_str() {
                "i8" => {
                    if let Ok(val) = processed_value.parse::<i8>() {
                        Variables::I8(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                "i16" => {
                    if let Ok(val) = processed_value.parse::<i16>() {
                        Variables::I16(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                "i32" => {
                    if let Ok(val) = processed_value.parse::<i32>() {
                        Variables::I32(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                "i64" => {
                    if let Ok(val) = processed_value.parse::<i64>() {
                        Variables::I64(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                "f32" => {
                    if let Ok(val) = processed_value.parse::<f32>() {
                        Variables::F32(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                "f64" => {
                    if let Ok(val) = processed_value.parse::<f64>() {
                        Variables::F64(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                _ => {
                    collected_errors.push(ErrTypes::TypeMismatch(*line));
                    return;
                }
            }
        } else {
            match var_type {
                "i8" => {
                    if let Ok(val) = processed_value.parse::<i8>() {
                        Variables::I8(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                "i16" => {
                    if let Ok(val) = processed_value.parse::<i16>() {
                        Variables::I16(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                "i32" => {
                    if let Ok(val) = processed_value.parse::<i32>() {
                        Variables::I32(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                "i64" => {
                    if let Ok(val) = processed_value.parse::<i64>() {
                        Variables::I64(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                "f32" => {
                    if let Ok(val) = processed_value.parse::<f32>() {
                        Variables::F32(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                "f64" => {
                    if let Ok(val) = processed_value.parse::<f64>() {
                        Variables::F64(Box::leak(var_name.clone().into_boxed_str()), val)
                    } else {
                        collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
                        return;
                    }
                }
                _ => Variables::MATH(var_name.clone(), processed_value.clone()),
            }
        }
    };

    ast.push(AST::VarAssign(new_var));
}

#[inline(always)]
fn validate_operand(
    operand: &str,
    _var_name: &str,
    collected_vars: &mut Vec<(String, &'static str)>,
    collected_errors: &mut Vec<ErrTypes>,
    line: i32,
) -> bool {
    let trimmed = operand.trim();
    if trimmed.is_empty() {
        collected_errors.push(ErrTypes::MissingValue(line));
        return false;
    }
    let cleaned = if trimmed.ends_with('f') || trimmed.ends_with('F') {
        &trimmed[..trimmed.len() - 1]
    } else {
        trimmed
    };
    if cleaned.parse::<f32>().is_ok() || collected_vars.iter().any(|(name, _)| name == cleaned) {
        true
    } else {
        collected_errors.push(ErrTypes::VarNotFound(line));
        false
    }
}

#[inline(always)]
fn format_operand(operand: &str) -> String {
    let trimmed = operand.trim();
    let cleaned = if trimmed.ends_with('f') || trimmed.ends_with('F') {
        &trimmed[..trimmed.len() - 1]
    } else {
        trimmed
    };
    if let Ok(num) = cleaned.parse::<f32>() {
        if num.fract().abs() < 1e-6 {
            format!("{}.0", num)
        } else {
            num.to_string()
        }
    } else {
        cleaned.to_string()
    }
}
