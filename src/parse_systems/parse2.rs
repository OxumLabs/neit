use super::parse3::parse3;
use super::AST;
use crate::parse_systems::Variables;
use crate::{err_system::err_types::ErrTypes, tok_system::tokens::Token};

#[allow(unused, non_snake_case)]
pub fn parse2(
    token: &Token,
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ast: &mut Vec<AST>,
    code: &String,
    collected_vars: &mut Vec<(String, &'static str)>,
    collected_errors: &mut Vec<ErrTypes>,
    line: &mut i32,
) {
    match token {
        Token::Iden(ref id) if id == "may" => {
            while let Some(Token::Space) = token_iter.peek() {
                token_iter.next();
            }
            let var_name = match token_iter.next() {
                Some(Token::Iden(name)) => name.clone(),
                _ => {
                    collected_errors.push(ErrTypes::UnknownCMD(*line));
                    return;
                }
            };

            if collected_vars.iter().any(|(name, _)| name == &var_name) {
                collected_errors.push(ErrTypes::VarAlreadyExists(*line));
                return;
            }

            while let Some(Token::Space) = token_iter.peek() {
                token_iter.next();
            }

            let mut found_eq = false;
            let mut found_math_op = false;
            let mut math_operator: Option<char> = None;
            if let Some(first) = token_iter.next() {
                match first {
                    Token::EqSign => {
                        found_eq = true;
                    }
                    Token::ADDOP => {
                        found_math_op = true;
                        math_operator = Some('+');
                    }
                    Token::SUBOP => {
                        found_math_op = true;
                        math_operator = Some('-');
                    }
                    Token::MULTIOP => {
                        found_math_op = true;
                        math_operator = Some('*');
                    }
                    Token::DIVOP => {
                        found_math_op = true;
                        math_operator = Some('/');
                    }
                    _ => {
                        collected_errors.push(ErrTypes::UnknownCMD(*line));
                        return;
                    }
                }
            } else {
                collected_errors.push(ErrTypes::UnknownCMD(*line));
                return;
            }

            while let Some(Token::Space) = token_iter.peek() {
                token_iter.next();
            }

            if found_eq && !found_math_op {
                if let Some(next_tok) = token_iter.peek() {
                    match next_tok {
                        Token::ADDOP => {
                            found_math_op = true;
                            math_operator = Some('+');
                            token_iter.next();
                        }
                        Token::SUBOP => {
                            found_math_op = true;
                            math_operator = Some('-');
                            token_iter.next();
                        }
                        Token::MULTIOP => {
                            found_math_op = true;
                            math_operator = Some('*');
                            token_iter.next();
                        }
                        Token::DIVOP => {
                            found_math_op = true;
                            math_operator = Some('/');
                            token_iter.next();
                        }
                        _ => {}
                    }
                }
            } else if found_math_op && !found_eq {
                while let Some(Token::Space) = token_iter.peek() {
                    token_iter.next();
                }
                if let Some(next_tok) = token_iter.next() {
                    if let Token::EqSign = next_tok {
                        found_eq = true;
                    } else {
                        collected_errors.push(ErrTypes::UnexpectedToken(*line));
                        return;
                    }
                } else {
                    collected_errors.push(ErrTypes::MissingValue(*line));
                    return;
                }
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
                    Token::LSmallBrac => {
                        raw_value.push('(');
                        token_iter.next();
                    }
                    Token::RSmallBracket => {
                        raw_value.push(')');
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

            if found_eq && found_math_op {
                match math_operator {
                    Some('+') => {
                        raw_value = format!("1+{}", raw_value);
                    }
                    Some('-') => {
                        raw_value = format!("1-{}", raw_value);
                    }
                    _ => {}
                }

                if raw_value.contains('+')
                    || raw_value.contains('-')
                    || raw_value.contains('*')
                    || raw_value.contains('/')
                {
                    let mut expr_tokens = Vec::new();
                    let mut current = String::new();
                    for c in raw_value.chars() {
                        if c == '+' || c == '-' || c == '*' || c == '/' {
                            if !current.trim().is_empty() {
                                expr_tokens.push(current.trim().to_string());
                            }
                            expr_tokens.push(c.to_string());
                            current.clear();
                        } else {
                            current.push(c);
                        }
                    }
                    if !current.trim().is_empty() {
                        expr_tokens.push(current.trim().to_string());
                    }
                    if !expr_tokens.is_empty() {
                        if let Some(first) = expr_tokens.first() {
                            if first == "+" || first == "-" || first == "*" || first == "/" {
                                collected_errors.push(ErrTypes::MissingValue(*line));
                                return;
                            }
                        }
                        if let Some(last) = expr_tokens.last() {
                            if last == "+" || last == "-" || last == "*" || last == "/" {
                                collected_errors.push(ErrTypes::MissingValue(*line));
                                return;
                            }
                        }
                        for i in 0..expr_tokens.len() - 1 {
                            if (expr_tokens[i] == "+" || expr_tokens[i] == "-" || expr_tokens[i] == "*" || expr_tokens[i] == "/")
                                && (expr_tokens[i + 1] == "+" || expr_tokens[i + 1] == "-" || expr_tokens[i + 1] == "*" || expr_tokens[i + 1] == "/")
                            {
                                collected_errors.push(ErrTypes::InvalidMathUsage(*line));
                                return;
                            }
                            if expr_tokens[i] == "/" && expr_tokens[i + 1].trim() == "0" {
                                collected_errors.push(ErrTypes::DivisionByZero(*line));
                                return;
                            }
                        }
                    }
                }
                let math_expr = raw_value.clone();
                let var = Variables::MATH(var_name.clone(), math_expr);
                if collected_vars.iter().all(|(name, _)| name != &var_name) {
                    collected_vars.push((var_name.clone(), "f32"));
                }
                ast.push(AST::Var(var));
            } else {
                let (is_quoted, mut processed_value) =
                    if raw_value.starts_with('\'') && raw_value.ends_with('\'') {
                        (true, raw_value[1..raw_value.len() - 1].to_string())
                    } else if raw_value.starts_with('"') && raw_value.ends_with('"') {
                        (true, raw_value[1..raw_value.len() - 1].to_string())
                    } else {
                        (false, raw_value.clone())
                    };

                if raw_value.starts_with('"') && raw_value.ends_with('"') {
                    let var_name_static = Box::leak(var_name.clone().into_boxed_str());
                    collected_vars.push((var_name.clone(), "str"));
                    ast.push(AST::Var(Variables::Str(var_name_static, processed_value)));
                } else if is_quoted {
                    if processed_value.chars().count() != 1 {
                        collected_errors.push(ErrTypes::CharVarLen(*line));
                        return;
                    }
                    let var_name_static = Box::leak(var_name.clone().into_boxed_str());
                    collected_vars.push((var_name.clone(), "ch"));
                    ast.push(AST::Var(Variables::Char(
                        var_name_static,
                        processed_value.chars().next().unwrap(),
                    )));
                } else {
                    let mut forced_type = None;
                    if processed_value.ends_with(')') {
                        if let Some(start) = processed_value.rfind('(') {
                            let potential = {
                                let slice = &processed_value[start + 1..processed_value.len() - 1];
                                slice.trim().to_string()
                            };
                            if potential == "i8" || potential == "i16" || potential == "i32" ||
                               potential == "i64" || potential == "f32" || potential == "f64" {
                                forced_type = Some(potential.clone());
                                let trimmed_value = {
                                    let slice = &processed_value[..start];
                                    slice.trim().to_string()
                                };
                                if trimmed_value.is_empty() {
                                    collected_errors.push(ErrTypes::MissingValue(*line));
                                    return;
                                }
                                processed_value = trimmed_value;
                            }
                        }
                    }
                    let var_name_static = Box::leak(var_name.clone().into_boxed_str());
                    if let Some(ty) = forced_type {
                        match ty.as_str() {
                            "i8" => {
                                if let Ok(val) = processed_value.parse::<i8>() {
                                    collected_vars.push((var_name.clone(), "i8"));
                                    ast.push(AST::Var(Variables::I8(var_name_static, val)));
                                } else {
                                    collected_errors.push(ErrTypes::VarNotFound(*line));
                                    return;
                                }
                            }
                            "i16" => {
                                if let Ok(val) = processed_value.parse::<i16>() {
                                    collected_vars.push((var_name.clone(), "i16"));
                                    ast.push(AST::Var(Variables::I16(var_name_static, val)));
                                } else {
                                    collected_errors.push(ErrTypes::VarNotFound(*line));
                                    return;
                                }
                            }
                            "i32" => {
                                if let Ok(val) = processed_value.parse::<i32>() {
                                    collected_vars.push((var_name.clone(), "i32"));
                                    ast.push(AST::Var(Variables::I32(var_name_static, val)));
                                } else {
                                    collected_errors.push(ErrTypes::VarNotFound(*line));
                                    return;
                                }
                            }
                            "i64" => {
                                if let Ok(val) = processed_value.parse::<i64>() {
                                    collected_vars.push((var_name.clone(), "i64"));
                                    ast.push(AST::Var(Variables::I64(var_name_static, val)));
                                } else {
                                    collected_errors.push(ErrTypes::VarNotFound(*line));
                                    return;
                                }
                            }
                            "f32" => {
                                if let Ok(val) = processed_value.parse::<f32>() {
                                    collected_vars.push((var_name.clone(), "f32"));
                                    ast.push(AST::Var(Variables::F32(var_name_static, val)));
                                } else {
                                    collected_errors.push(ErrTypes::VarNotFound(*line));
                                    return;
                                }
                            }
                            "f64" => {
                                if let Ok(val) = processed_value.parse::<f64>() {
                                    collected_vars.push((var_name.clone(), "f64"));
                                    ast.push(AST::Var(Variables::F64(var_name_static, val)));
                                } else {
                                    collected_errors.push(ErrTypes::VarNotFound(*line));
                                    return;
                                }
                            }
                            _ => {}
                        }
                    } else {
                        if let Ok(val) = processed_value.parse::<i8>() {
                            collected_vars.push((var_name.clone(), "i8"));
                            ast.push(AST::Var(Variables::I8(var_name_static, val)));
                        } else if let Ok(val) = processed_value.parse::<i16>() {
                            collected_vars.push((var_name.clone(), "i16"));
                            ast.push(AST::Var(Variables::I16(var_name_static, val)));
                        } else if let Ok(val) = processed_value.parse::<i32>() {
                            collected_vars.push((var_name.clone(), "i32"));
                            ast.push(AST::Var(Variables::I32(var_name_static, val)));
                        } else if let Ok(val) = processed_value.parse::<i64>() {
                            collected_vars.push((var_name.clone(), "i64"));
                            ast.push(AST::Var(Variables::I64(var_name_static, val)));
                        } else if let Ok(val) = processed_value.parse::<f32>() {
                            collected_vars.push((var_name.clone(), "f32"));
                            ast.push(AST::Var(Variables::F32(var_name_static, val)));
                        } else if let Ok(val) = processed_value.parse::<f64>() {
                            collected_vars.push((var_name.clone(), "f64"));
                            ast.push(AST::Var(Variables::F64(var_name_static, val)));
                        } else if collected_vars.iter().any(|(name, _)| name == &processed_value) {
                            collected_vars.push((var_name.clone(), "ref"));
                            ast.push(AST::Var(Variables::REF(var_name_static, processed_value)));
                        } else {
                            collected_errors.push(ErrTypes::VarNotFound(*line));
                            return;
                        }
                    }
                }
            }
        }
        Token::EOL => {
            collected_errors.push(ErrTypes::UnexpectedToken(*line));
            return;
        }
        other => {
            parse3(token, token_iter, ast, code, collected_vars, collected_errors, line);
        }
    }
}
