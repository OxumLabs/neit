use crate::{err_system::err_types::ErrTypes, tok_system::tokens::Token};
use crate::parse_systems::{Variables, COLLECTED_VARS};
use super::parse3::parse3;
use super::{AST, COLLECTED_ERRORS, LINE};

#[allow(unused)]
pub fn parse2(
    token: &Token,
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ast: &mut Vec<AST>,
    code: &String
) {
    match token {
        Token::Iden(ref id) if id == "may" => {
            while let Some(Token::Space) = token_iter.peek() { token_iter.next(); }
            let var_name = match token_iter.next() {
                Some(Token::Iden(name)) => name.clone(),
                _ => {
                    if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                        unsafe { errors.push(ErrTypes::UnknownCMD(LINE)) };
                    }
                    return;
                }
            };
            if COLLECTED_VARS.lock().unwrap().iter().any(|(name, _)| name == &var_name) {
                if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                    unsafe { errors.push(ErrTypes::VarAlreadyExists(LINE)) };
                }
                return;
            }
            while let Some(Token::Space) = token_iter.peek() { token_iter.next(); }
            let mut found_eq = false;
            let mut found_math_op = false;
            let mut math_operator: Option<char> = None;
            if let Some(first) = token_iter.next() {
                match first {
                    Token::EqSign => { found_eq = true; }
                    Token::ADDOP => { found_math_op = true; math_operator = Some('+'); }
                    Token::SUBOP => { found_math_op = true; math_operator = Some('-'); }
                    Token::MULTIOP => { found_math_op = true; math_operator = Some('*'); }
                    Token::DIVOP => { found_math_op = true; math_operator = Some('/'); }
                    _ => {
                        if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                            unsafe { errors.push(ErrTypes::UnknownCMD(LINE)) };
                        }
                        return;
                    }
                }
            } else {
                if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                    unsafe { errors.push(ErrTypes::UnknownCMD(LINE)) };
                }
                return;
            }
            while let Some(Token::Space) = token_iter.peek() { token_iter.next(); }
            if found_eq && !found_math_op {
                if let Some(next_tok) = token_iter.peek() {
                    match next_tok {
                        Token::ADDOP => { found_math_op = true; math_operator = Some('+'); token_iter.next(); }
                        Token::SUBOP => { found_math_op = true; math_operator = Some('-'); token_iter.next(); }
                        Token::MULTIOP => { found_math_op = true; math_operator = Some('*'); token_iter.next(); }
                        Token::DIVOP => { found_math_op = true; math_operator = Some('/'); token_iter.next(); }
                        _ => {}
                    }
                }
            } else if found_math_op && !found_eq {
                while let Some(Token::Space) = token_iter.peek() { token_iter.next(); }
                if let Some(next_tok) = token_iter.next() {
                    if let Token::EqSign = next_tok { found_eq = true; }
                    else {
                        if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                            unsafe { errors.push(ErrTypes::UnexpectedToken(LINE)) };
                        }
                       	return;
                    }
                } else {
                    if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                        unsafe { errors.push(ErrTypes::MissingValue(LINE)) };
                    }
                    return;
                }
            }
            while let Some(Token::Space) = token_iter.peek() { token_iter.next(); }
            let mut raw_value = String::new();
            while let Some(tok) = token_iter.peek() {
                match tok {
                    Token::EOL | Token::EOF => break,
                    Token::Space => { token_iter.next(); }
                    Token::Iden(val) => { raw_value.push_str(val); token_iter.next(); }
                    Token::ADDOP => { raw_value.push('+'); token_iter.next(); }
                    Token::SUBOP => { raw_value.push('-'); token_iter.next(); }
                    Token::MULTIOP => { raw_value.push('*'); token_iter.next(); }
                    Token::DIVOP => { raw_value.push('/'); token_iter.next(); }
                    _ => { token_iter.next(); }
                }
            }
            if raw_value.trim().is_empty() {
                if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                    unsafe { errors.push(ErrTypes::MissingValue(LINE)) };
                }
                return;
            }
            if (found_eq && found_math_op) || raw_value.contains('+') || raw_value.contains('-') || raw_value.contains('*') || raw_value.contains('/') {
                let mut expr_tokens = Vec::new();
                let mut current = String::new();
                for c in raw_value.chars() {
                    if c == '+' || c == '-' || c == '*' || c == '/' {
                        if !current.trim().is_empty() { expr_tokens.push(current.trim().to_string()); }
                        expr_tokens.push(c.to_string());
                        current.clear();
                    } else {
                        current.push(c);
                    }
                }
                if !current.trim().is_empty() { expr_tokens.push(current.trim().to_string()); }
                if !expr_tokens.is_empty() {
                    if let Some(first) = expr_tokens.first() {
                        if first == "+" || first == "-" || first == "*" || first == "/" {
                            if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                                unsafe { errors.push(ErrTypes::MissingValue(LINE)) };
                            }
                            return;
                        }
                    }
                    if let Some(last) = expr_tokens.last() {
                        if last == "+" || last == "-" || last == "*" || last == "/" {
                            if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                                unsafe { errors.push(ErrTypes::MissingValue(LINE)) };
                            }
                            return;
                        }
                    }
                    for i in 0..expr_tokens.len()-1 {
                        if (expr_tokens[i] == "+" || expr_tokens[i] == "-" || expr_tokens[i] == "*" || expr_tokens[i] == "/") &&
                           (expr_tokens[i+1] == "+" || expr_tokens[i+1] == "-" || expr_tokens[i+1] == "*" || expr_tokens[i+1] == "/") {
                            if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                                unsafe { errors.push(ErrTypes::InvalidMathUsage(LINE)) };
                            }
                            return;
                        }
                        if expr_tokens[i] == "/" && expr_tokens[i+1].trim() == "0" {
                            if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                                unsafe { errors.push(ErrTypes::DivisionByZero(LINE)) };
                            }
                            return;
                        }
                    }
                }
                let math_expr = raw_value.clone();
                let var = Variables::MATH(var_name.clone(), math_expr);
                if COLLECTED_VARS.lock().unwrap().iter().all(|(name, _)| name != &var_name) {
                    COLLECTED_VARS.lock().unwrap().push((var_name.clone(), "f32"));
                }
                ast.push(AST::Var(var));
            } else {
                let (is_quoted, processed_value) = if raw_value.starts_with('\'') && raw_value.ends_with('\'') {
                    (true, raw_value[1..raw_value.len()-1].to_string())
                } else if raw_value.starts_with('"') && raw_value.ends_with('"') {
                    (true, raw_value[1..raw_value.len()-1].to_string())
                } else {
                    (false, raw_value.clone())
                };
                if raw_value.starts_with('"') && raw_value.ends_with('"') {
                    let var_name_static = Box::leak(var_name.clone().into_boxed_str());
                    COLLECTED_VARS.lock().unwrap().push((var_name.clone(), "str"));
                    ast.push(AST::Var(Variables::Str(var_name_static, processed_value)));
                } else if is_quoted {
                    if processed_value.chars().count() != 1 {
                        if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                            unsafe { errors.push(ErrTypes::CharVarLen(LINE)) };
                        }
                        return;
                    }
                    let var_name_static = Box::leak(var_name.clone().into_boxed_str());
                    COLLECTED_VARS.lock().unwrap().push((var_name.clone(), "ch"));
                    ast.push(AST::Var(Variables::Char(var_name_static, processed_value.chars().next().unwrap())));
                } else {
                    let var_name_static = Box::leak(var_name.clone().into_boxed_str());
                    if let Ok(val) = processed_value.parse::<i8>() {
                        COLLECTED_VARS.lock().unwrap().push((var_name.clone(), "i8"));
                        ast.push(AST::Var(Variables::I8(var_name_static, val)));
                    } else if let Ok(val) = processed_value.parse::<i16>() {
                        COLLECTED_VARS.lock().unwrap().push((var_name.clone(), "i16"));
                        ast.push(AST::Var(Variables::I16(var_name_static, val)));
                    } else if let Ok(val) = processed_value.parse::<i32>() {
                        COLLECTED_VARS.lock().unwrap().push((var_name.clone(), "i32"));
                        ast.push(AST::Var(Variables::I32(var_name_static, val)));
                    } else if let Ok(val) = processed_value.parse::<i64>() {
                        COLLECTED_VARS.lock().unwrap().push((var_name.clone(), "i64"));
                        ast.push(AST::Var(Variables::I64(var_name_static, val)));
                    } else if let Ok(val) = processed_value.parse::<f32>() {
                        COLLECTED_VARS.lock().unwrap().push((var_name.clone(), "f32"));
                        ast.push(AST::Var(Variables::F32(var_name_static, val)));
                    } else if let Ok(val) = processed_value.parse::<f64>() {
                        COLLECTED_VARS.lock().unwrap().push((var_name.clone(), "f64"));
                        ast.push(AST::Var(Variables::F64(var_name_static, val)));
                    } else if COLLECTED_VARS.lock().unwrap().iter().any(|(name, _)| name == &processed_value) {
                        COLLECTED_VARS.lock().unwrap().push((var_name.clone(), "ref"));
                        ast.push(AST::Var(Variables::REF(var_name_static, processed_value)));
                    } else {
                        if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                            unsafe { errors.push(ErrTypes::VarNotFound(LINE)) };
                        }
                        return;
                    }
                }
            }
        }
        Token::EOL => {
            if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                unsafe { errors.push(ErrTypes::UnexpectedToken(LINE)) };
            }
            return;
        }
        _ => {
            //println!("├──[!] Second Parser Part was unable to check the AST , trying next parser part");

            parse3(token, token_iter, ast, code);
        }
    }
}
