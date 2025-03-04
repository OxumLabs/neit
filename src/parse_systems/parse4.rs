use super::{AST, COLLECTED_VARS, LINE};
use crate::{
    err_system::err_types::ErrTypes,
    parse_systems::{Variables, COLLECTED_ERRORS},
    tok_system::tokens::Token,
};
#[allow(unused)]
pub fn parse4(
    token: &Token,
    token_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    ast: &mut Vec<AST>,
    _code: &String,
) {
    let var_name = if let Token::Iden(ref name) = token {
        name.clone()
    } else {
        return;
    };

    let var_info_option = COLLECTED_VARS
        .lock()
        .unwrap()
        .iter()
        .find(|(name, _)| name == &var_name)
        .cloned();
    if var_info_option.is_none() {
        unsafe { COLLECTED_ERRORS.lock().unwrap().push(ErrTypes::VarNotFound(LINE)) };
        return;
    }
    let (_, var_type) = var_info_option.unwrap();
    if var_type.contains("const;") {
        unsafe { COLLECTED_ERRORS.lock().unwrap().push(ErrTypes::VarISConst(LINE)) };
        return;
    }

    while let Some(Token::Space) = token_iter.peek() {
        token_iter.next();
    }

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
                while let Some(Token::Space) = token_iter.peek() {
                    token_iter.next();
                }
                if let Some(Token::EqSign) = token_iter.next() {
                } else {
                    COLLECTED_ERRORS
                        .lock()
                        .unwrap()
                        .push(ErrTypes::MissingOperator(unsafe { LINE }));
                    return;
                }
            }
            _ => {
                COLLECTED_ERRORS
                    .lock()
                    .unwrap()
                    .push(ErrTypes::UnexpectedToken(unsafe { LINE }));
                return;
            }
        }
    } else {
        COLLECTED_ERRORS
            .lock()
            .unwrap()
            .push(ErrTypes::MissingOperator(unsafe { LINE }));
        return;
    }
    while let Some(Token::Space) = token_iter.peek() {
        token_iter.next();
    }

    let mut raw_value = String::new();
    while let Some(tok) = token_iter.peek() {
        match tok {
            Token::EOL | Token::EOF => break,
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
        COLLECTED_ERRORS
            .lock()
            .unwrap()
            .push(ErrTypes::MissingValue(unsafe { LINE }));
        return;
    }

    let mut final_expr = {
        let mut result = String::new();
        let mut current_operand = String::new();
        for c in raw_value.chars() {
            if c == '+' || c == '-' || c == '*' || c == '/' {
                if !current_operand.is_empty() {
                    if !validate_operand(&current_operand, &var_name) {
                        return;
                    }
                    let formatted = format_operand(&current_operand);
                    result.push_str(&formatted);
                    current_operand.clear();
                }
                result.push(c);
            } else {
                current_operand.push(c);
            }
        }
        if !current_operand.is_empty() {
            if !validate_operand(&current_operand, &var_name) {
                return;
            }
            let formatted = format_operand(&current_operand);
            result.push_str(&formatted);
        }
        result
    };

    if let Some(op) = compound_operator {
        final_expr = format!("{}{}{}", var_name, op, final_expr);
    }

    if final_expr.contains('+')
        || final_expr.contains('-')
        || final_expr.contains('*')
        || final_expr.contains('/')
    {
        let new_var = Variables::MATH(var_name.clone(), final_expr.clone());
        ast.push(AST::VarAssign(new_var));
        return;
    }

    let new_var: Variables = if final_expr.starts_with('\"') && final_expr.ends_with('\"') {
        let processed = final_expr[1..final_expr.len() - 1].to_string();
        if var_type == "str" {
            Variables::Str(Box::leak(var_name.clone().into_boxed_str()), processed)
        } else {
            COLLECTED_ERRORS
                .lock()
                .unwrap()
                .push(ErrTypes::TypeMismatch(unsafe { LINE }));
            return;
        }
    } else if final_expr.starts_with('\'') && final_expr.ends_with('\'') {
        let processed = final_expr[1..final_expr.len() - 1].to_string();
        if processed.chars().count() != 1 {
            COLLECTED_ERRORS
                .lock()
                .unwrap()
                .push(ErrTypes::CharVarLen(unsafe { LINE }));
            return;
        }
        if var_type == "ch" {
            Variables::Char(
                Box::leak(var_name.clone().into_boxed_str()),
                processed.chars().next().unwrap(),
            )
        } else {
            COLLECTED_ERRORS
                .lock()
                .unwrap()
                .push(ErrTypes::TypeMismatch(unsafe { LINE }));
            return;
        }
    } else {
        match var_type {
            "i8" => {
                if let Ok(val) = final_expr.parse::<i8>() {
                    Variables::I8(Box::leak(var_name.clone().into_boxed_str()), val)
                } else {
                    COLLECTED_ERRORS
                        .lock()
                        .unwrap()
                        .push(ErrTypes::InvalidNumberFormat(unsafe { LINE }));
                    return;
                }
            }
            "i16" => {
                if let Ok(val) = final_expr.parse::<i16>() {
                    Variables::I16(Box::leak(var_name.clone().into_boxed_str()), val)
                } else {
                    COLLECTED_ERRORS
                        .lock()
                        .unwrap()
                        .push(ErrTypes::InvalidNumberFormat(unsafe { LINE }));
                    return;
                }
            }
            "i32" => {
                if let Ok(val) = final_expr.parse::<i32>() {
                    Variables::I32(Box::leak(var_name.clone().into_boxed_str()), val)
                } else {
                    COLLECTED_ERRORS
                        .lock()
                        .unwrap()
                        .push(ErrTypes::InvalidNumberFormat(unsafe { LINE }));
                    return;
                }
            }
            "i64" => {
                if let Ok(val) = final_expr.parse::<i64>() {
                    Variables::I64(Box::leak(var_name.clone().into_boxed_str()), val)
                } else {
                    COLLECTED_ERRORS
                        .lock()
                        .unwrap()
                        .push(ErrTypes::InvalidNumberFormat(unsafe { LINE }));
                    return;
                }
            }
            "f32" => {
                if let Ok(val) = final_expr.parse::<f32>() {
                    Variables::F32(Box::leak(var_name.clone().into_boxed_str()), val)
                } else {
                    COLLECTED_ERRORS
                        .lock()
                        .unwrap()
                        .push(ErrTypes::InvalidNumberFormat(unsafe { LINE }));
                    return;
                }
            }
            "f64" => {
                if let Ok(val) = final_expr.parse::<f64>() {
                    Variables::F64(Box::leak(var_name.clone().into_boxed_str()), val)
                } else {
                    COLLECTED_ERRORS
                        .lock()
                        .unwrap()
                        .push(ErrTypes::InvalidNumberFormat(unsafe { LINE }));
                    return;
                }
            }
            _ => Variables::MATH(var_name.clone(), final_expr.clone()),
        }
    };

    ast.push(AST::VarAssign(new_var));
}

fn validate_operand(operand: &str, _var_name: &str) -> bool {
    let trimmed = operand.trim();
    if trimmed.is_empty() {
        COLLECTED_ERRORS
            .lock()
            .unwrap()
            .push(ErrTypes::MissingValue(unsafe { LINE }));
        return false;
    }
    let cleaned = if trimmed.ends_with('f') || trimmed.ends_with('F') {
        &trimmed[..trimmed.len() - 1]
    } else {
        trimmed
    };
    if cleaned.parse::<f32>().is_ok() {
        return true;
    }
    if COLLECTED_VARS
        .lock()
        .unwrap()
        .iter()
        .any(|(name, _)| name == cleaned)
    {
        return true;
    }
    COLLECTED_ERRORS
        .lock()
        .unwrap()
        .push(ErrTypes::VarNotFound(unsafe { LINE }));
    false
}

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
            format!("{}", num)
        }
    } else {
        cleaned.to_string()
    }
}
