use crate::{
    err_system::err_types::ErrTypes,
    tok_system::tokens::Token,
};
use super::{ChildCond, Condition, LogicalJoin, Operand, CondToks};

/// Parses a raw condition (vector of tokens) into a `Condition` structure.
///
/// # Arguments
/// - `raw_cond`: The tokens representing the condition.
/// - `collected_errors`: Mutable reference to the error vector for reporting issues.
/// - `collected_vars`: Immutable reference to the collected variables (name and type).
/// - `line`: The current line number (used for error reporting).
pub fn parse_condition(
    raw_cond: &Vec<Token>,
    collected_errors: &mut Vec<ErrTypes>,
    collected_vars: &Vec<(String, &'static str)>,
    line: i32,
) -> Condition {
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
                } else if let Some((_, var_type)) = collected_vars.iter().find(|(name, _)| name == s) {
                    (Operand::Variable(s.clone()), *var_type)
                } else {
                    collected_errors.push(ErrTypes::VarNotFound(line));
                    (Operand::Variable(s.clone()), "unknown")
                }
            }
            _ => continue,
        };
        
        // Skip any spaces.
        while let Some(Token::Space) = tokens.peek() {
            tokens.next();
        }
        
        // Parse operator.
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
                    collected_errors.push(ErrTypes::MissingOperator(line));
                    continue;
                }
            }
            _ => continue,
        };
        
        while let Some(Token::Space) = tokens.peek() {
            tokens.next();
        }
        
        // Parse right operand.
        let (right_operand, right_type) = match tokens.next() {
            Some(Token::Iden(s)) => {
                if s.starts_with("\"") && s.ends_with("\"") {
                    (Operand::Literal(s[1..s.len() - 1].to_string()), "str")
                } else if let Ok(n) = s.parse::<f64>() {
                    (Operand::Numeric(n), "f64")
                } else if let Some((_, var_type)) = collected_vars.iter().find(|(name, _)| name == s) {
                    (Operand::Variable(s.clone()), *var_type)
                } else {
                    collected_errors.push(ErrTypes::VarNotFound(line));
                    (Operand::Variable(s.clone()), "unknown")
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
            collected_errors.push(ErrTypes::TypeMismatch(line));
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
