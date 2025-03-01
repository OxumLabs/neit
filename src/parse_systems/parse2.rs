use crate::{err_system::err_types::ErrTypes, tok_system::tokens::Token};
use crate::parse_systems::{Variables, COLLECTED_VARS};
use super::{AST, COLLECTED_ERRORS, LINE};

pub fn parse2(
    token: &Token,
    token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ast: &mut Vec<AST>
) {
    println!("parse2 called");
    println!("Initial token: {:?}", token);

    match token {
        Token::Iden(ref id) if id == "may" => {
            println!("Token matched 'may' command.");

            while let Some(Token::Space) = token_iter.peek() {
                println!("Skipping space before variable name.");
                token_iter.next();
            }
            
            let var_name = match token_iter.next() {
                Some(Token::Iden(name)) => {
                    println!("Found variable name: {}", name);
                    name
                },
                other => {
                    println!("Error: Expected variable name identifier, but found: {:?}", other);
                    unsafe {
                        if let Ok(mut e) = COLLECTED_ERRORS.lock() {
                            e.push(ErrTypes::UnknownCMD(LINE));
                        }
                    }
                    return;
                }
            };

            // Check if variable already exists
            if COLLECTED_VARS.lock().unwrap().contains(&var_name) {
                println!("Error: Variable '{}' already exists.", var_name);
                unsafe {
                    if let Ok(mut e) = COLLECTED_ERRORS.lock() {
                        e.push(ErrTypes::VarAlreadyExists(LINE));
                    }
                }
                return;
            }

            while let Some(Token::Space) = token_iter.peek() {
                println!("Skipping space before '=' token.");
                token_iter.next();
            }
            
            if !matches!(token_iter.next(), Some(Token::EqSign)) {
                println!("Error: Expected '=' token but did not find it.");
                unsafe {
                    if let Ok(mut e) = COLLECTED_ERRORS.lock() {
                        e.push(ErrTypes::UnknownCMD(LINE));
                    }
                }
                return;
            } else {
                println!("Found '=' token.");
            }

            while let Some(Token::Space) = token_iter.peek() {
                println!("Skipping space before variable value.");
                token_iter.next();
            }

            let raw_value = match token_iter.next() {
                Some(Token::Iden(val)) => {
                    println!("Found raw value token: {}", val);
                    val.clone()
                },
                other => {
                    println!("Error: Expected variable value identifier, but found: {:?}", other);
                    unsafe {
                        if let Ok(mut e) = COLLECTED_ERRORS.lock() {
                            e.push(ErrTypes::UnknownCMD(LINE));
                        }
                    }
                    return;
                }
            };

            let var_value = if raw_value.starts_with('\'') && raw_value.ends_with('\'') && raw_value.len() >= 2 {
                println!("Value {} is wrapped in quotes. Stripping quotes.", raw_value);
                raw_value[1..raw_value.len()-1].to_string()
            } else {
                println!("Value {} is not wrapped in quotes.", raw_value);
                raw_value
            };

            let var_name_static = Box::leak(var_name.clone().into_boxed_str());
            println!("Final variable name (static): {}", var_name_static);
            println!("Final variable value after processing: {}", var_value);

            let var = if let Ok(val) = var_value.parse::<i8>() {
                println!("Parsed value as i8: {}", val);
                Variables::I8(var_name_static, val)
            } else if let Ok(val) = var_value.parse::<i16>() {
                println!("Parsed value as i16: {}", val);
                Variables::I16(var_name_static, val)
            } else if let Ok(val) = var_value.parse::<i32>() {
                println!("Parsed value as i32: {}", val);
                Variables::I32(var_name_static, val)
            } else if let Ok(val) = var_value.parse::<i64>() {
                println!("Parsed value as i64: {}", val);
                Variables::I64(var_name_static, val)
            } else if let Ok(val) = var_value.parse::<f32>() {
                println!("Parsed value as f32: {}", val);
                Variables::F32(var_name_static, val)
            } else if let Ok(val) = var_value.parse::<f64>() {
                println!("Parsed value as f64: {}", val);
                Variables::F64(var_name_static, val)
            } else if COLLECTED_VARS.lock().unwrap().contains(&var_value) {
                println!("Value {} is a reference to another variable.", var_value);
                Variables::REF(var_name_static, var_value)
            } else if var_value.len() == 1 {
                println!("Parsed value as char: {}", var_value);
                Variables::Char(var_name_static, var_value.chars().next().unwrap())
            } else {
                println!("Error: Unsupported variable type or unknown reference for value: {}", var_value);
                unsafe {
                    if let Ok(mut e) = COLLECTED_ERRORS.lock() {
                        e.push(ErrTypes::UnsupportedVarType(LINE));
                    }
                }
                return;
            };

            println!("Variable parsed successfully: {:?}", var);
            COLLECTED_VARS.lock().unwrap().push(var_name.clone());
            ast.push(AST::Var(var));
            println!("AST updated with new variable. Current AST length: {}", ast.len());
        }
        other => {
            println!("Error: Token did not match 'may'. Received token: {:?}", other);
            unsafe {
                if let Ok(mut e) = COLLECTED_ERRORS.lock() {
                    e.push(ErrTypes::UnknownCMD(LINE));
                }
            }
        }
    }
    println!("parse2 completed.");
}
