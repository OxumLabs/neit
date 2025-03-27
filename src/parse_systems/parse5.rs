use super::AST;
use crate::{
    err_system::err_types::ErrTypes,
    parse_systems::Variables,
    tok_system::tokens::Token,
};

#[inline(always)]
pub fn parse5(
    token: &Token,
    token_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    ast: &mut Vec<AST>,
    _code: &String,
    collected_vars: &mut Vec<(String, &'static str)>,
    collected_errors: &mut Vec<ErrTypes>,
    line: &mut i32,
) {
    // Expect the first token to be "const"
    let cmd = match token {
        Token::Iden(cmd) => cmd,
        _ => return,
    };
    if cmd != "const" {
        return;
    }

    // Skip spaces after "const"
    while matches!(token_iter.peek(), Some(Token::Space)) {
        token_iter.next();
    }

    // Get variable name.
    let var_name = match token_iter.next() {
        Some(Token::Iden(name)) => name.clone(),
        _ => {
            collected_errors.push(ErrTypes::UnknownCMD(*line));
            return;
        }
    };

    // Avoid duplicate declarations.
    if collected_vars.iter().any(|(name, _)| name == &var_name) {
        collected_errors.push(ErrTypes::VarAlreadyExists(*line));
        return;
    }
    
    // Skip spaces before assignment operator.
    while matches!(token_iter.peek(), Some(Token::Space)) {
        token_iter.next();
    }

    // Process assignment operator.
    let mut compound_operator: Option<char> = None;
    if let Some(op_token) = token_iter.next() {
        match op_token {
            Token::EqSign => { }
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
                    // compound assignment confirmed
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
            Token::EOL | Token::EOF => {
                *line += 1;
                break
            },
            Token::Space => { token_iter.next(); },
            Token::Iden(val) => { raw_value.push_str(val); token_iter.next(); },
            Token::ADDOP => { raw_value.push('+'); token_iter.next(); },
            Token::SUBOP => { raw_value.push('-'); token_iter.next(); },
            Token::MULTIOP => { raw_value.push('*'); token_iter.next(); },
            Token::DIVOP => { raw_value.push('/'); token_iter.next(); },
            _ => { token_iter.next(); },
        }
    }
    if raw_value.trim().is_empty() {
        collected_errors.push(ErrTypes::MissingValue(*line));
        return;
    }

    // Process operands and rebuild expression.
    let final_expr = if (raw_value.starts_with('\"') && raw_value.ends_with('\"'))
        || (raw_value.starts_with('\'') && raw_value.ends_with('\'')) 
    {
        raw_value
    } else {
        let mut result = String::new();
        let mut current_operand = String::new();
        for c in raw_value.chars() {
            if "+-*/".contains(c) {
                if !current_operand.is_empty() {
                    if !validate_operand(&current_operand, &var_name, collected_vars, collected_errors, *line) {
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
            if !validate_operand(&current_operand, &var_name, collected_vars, collected_errors, *line) {
                return;
            }
            result.push_str(&format_operand(&current_operand));
        }
        result
    };

    let final_expr = if let Some(op) = compound_operator {
        format!("{}{}{}", var_name, op, final_expr)
    } else {
        final_expr
    };

    // Create constant variable.
    let new_var: Variables = if final_expr.starts_with('\"') && final_expr.ends_with('\"') {
        let processed = final_expr[1..final_expr.len()-1].to_string();
        Variables::Str(Box::leak(var_name.clone().into_boxed_str()), processed)
    } else if final_expr.starts_with('\'') && final_expr.ends_with('\'') {
        let processed = final_expr[1..final_expr.len()-1].to_string();
        if processed.chars().count() != 1 {
            collected_errors.push(ErrTypes::CharVarLen(*line));
            return;
        }
        Variables::Char(Box::leak(var_name.clone().into_boxed_str()), processed.chars().next().unwrap())
    } else if final_expr.contains('+') || final_expr.contains('-') || final_expr.contains('*') || final_expr.contains('/') {
        Variables::MATH(var_name.clone(), final_expr.clone())
    } else {
        if let Ok(val) = final_expr.parse::<i32>() {
            Variables::I32(Box::leak(var_name.clone().into_boxed_str()), val)
        } else if let Ok(val) = final_expr.parse::<f32>() {
            Variables::F32(Box::leak(var_name.clone().into_boxed_str()), val)
        } else {
            collected_errors.push(ErrTypes::InvalidNumberFormat(*line));
            return;
        }
    };

    let const_type = match &new_var {
        Variables::I8(_, _)   => "const;i8",
        Variables::I16(_, _)  => "const;i16",
        Variables::I32(_, _)  => "const;i32",
        Variables::I64(_, _)  => "const;i64",
        Variables::F32(_, _)  => "const;f32",
        Variables::F64(_, _)  => "const;f64",
        Variables::Str(_, _)  => "const;str",
        Variables::Char(_, _) => "const;ch",
        Variables::MATH(_, _) => "const;f32",
        Variables::REF(_, _)  => "const;ref",
    };

    collected_vars.push((var_name.clone(), const_type));
    ast.push(AST::Var(new_var));
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
        &trimmed[..trimmed.len()-1]
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
        &trimmed[..trimmed.len()-1]
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
