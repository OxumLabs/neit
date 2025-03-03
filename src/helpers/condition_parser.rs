use crate::{
    err_system::err_types::ErrTypes,
    parse_systems::{COLLECTED_ERRORS, COLLECTED_VARS, LINE},
    tok_system::tokens::Token,
};
use super::{ChildCond, Condition, LogicalJoin, Operand, CondToks};

pub fn parse_condition(raw_cond: &Vec<Token>) -> Condition {
    let mut child_conditions = Vec::new();
    let mut tokens = raw_cond.iter().peekable();
    while let Some(tok) = tokens.peek() {
        if let Token::Space = tok {
            tokens.next();
            continue;
        }
        let (left_operand, left_type) = match tokens.next() {
            Some(Token::Iden(s)) => {
                if s.starts_with("\"") && s.ends_with("\"") {
                    (Operand::Literal(s[1..s.len() - 1].to_string()), "str")
                } else if let Ok(n) = s.parse::<f64>() {
                    (Operand::Numeric(n), "f64")
                } else {
                    let collected_vars = COLLECTED_VARS.lock().unwrap();
                    if let Some((_, var_type)) = collected_vars.iter().find(|(name, _)| name == s) {
                        (Operand::Variable(s.clone()), *var_type)
                    } else {
                        if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                            unsafe { errors.push(ErrTypes::VarNotFound(LINE)) };
                        }
                        (Operand::Variable(s.clone()), "unknown")
                    }
                }
            }
            _ => continue,
        };
        while let Some(Token::Space) = tokens.peek() {
            tokens.next();
        }
        let operator = match tokens.next() {
            Some(Token::DoubleEqSign) => CondToks::Equal,
            Some(Token::EqSign) => CondToks::Equal,
            Some(Token::GreaterThan) => CondToks::GreaterThan,
            Some(Token::LessThan) => CondToks::LessThan,
            Some(Token::Not) => {
                if let Some(Token::EqSign) = tokens.peek() {
                    tokens.next();
                    CondToks::NotEqual
                } else {
                    if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                        unsafe { errors.push(ErrTypes::MissingOperator(LINE)) };
                    }
                    continue;
                }
            }
            _ => continue,
        };
        while let Some(Token::Space) = tokens.peek() {
            tokens.next();
        }
        let (right_operand, right_type) = match tokens.next() {
            Some(Token::Iden(s)) => {
                if s.starts_with("\"") && s.ends_with("\"") {
                    (Operand::Literal(s[1..s.len() - 1].to_string()), "str")
                } else if let Ok(n) = s.parse::<f64>() {
                    (Operand::Numeric(n), "f64")
                } else {
                    let collected_vars = COLLECTED_VARS.lock().unwrap();
                    if let Some((_, var_type)) = collected_vars.iter().find(|(name, _)| name == s) {
                        (Operand::Variable(s.clone()), *var_type)
                    } else {
                        if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                            unsafe { errors.push(ErrTypes::VarNotFound(LINE)) };
                        }
                        (Operand::Variable(s.clone()), "unknown")
                    }
                }
            }
            _ => continue,
        };
        let valid_combinations = [
            ("i8", "i8"),
            ("i8", "i16"),
            ("i8", "i32"),
            ("i8", "i64"),
            ("i8", "f32"),
            ("i8", "f64"),
            ("i16", "i8"),
            ("i16", "i16"),
            ("i16", "i32"),
            ("i16", "i64"),
            ("i16", "f32"),
            ("i16", "f64"),
            ("i32", "i8"),
            ("i32", "i16"),
            ("i32", "i32"),
            ("i32", "i64"),
            ("i32", "f32"),
            ("i32", "f64"),
            ("i64", "i8"),
            ("i64", "i16"),
            ("i64", "i32"),
            ("i64", "i64"),
            ("i64", "f32"),
            ("i64", "f64"),
            ("f32", "i8"),
            ("f32", "i16"),
            ("f32", "i32"),
            ("f32", "i64"),
            ("f32", "f32"),
            ("f32", "f64"),
            ("f64", "i8"),
            ("f64", "i16"),
            ("f64", "i32"),
            ("f64", "i64"),
            ("f64", "f32"),
            ("f64", "f64"),
            ("str", "str"),
            ("ch", "ch"),
        ];
        let is_valid = valid_combinations
            .iter()
            .any(|&(lt, rt)| lt == left_type && rt == right_type);
        if !is_valid {
            if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                unsafe { errors.push(ErrTypes::TypeMismatch(LINE)) };
            }
            continue;
        }
        while let Some(Token::Space) = tokens.peek() {
            tokens.next();
        }
        let joiner = if let Some(token) = tokens.peek() {
            match token {
                Token::And => {
                    tokens.next();
                    Some(LogicalJoin::And)
                }
                Token::Or => {
                    tokens.next();
                    Some(LogicalJoin::Or)
                }
                _ => None,
            }
        } else {
            None
        };
        child_conditions.push(ChildCond {
            left: left_operand,
            operator,
            right: right_operand,
            joiner,
        });
    }
    Condition { child_conditions }
}
