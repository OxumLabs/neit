use super::{ChildCond, CondToks, Condition, LogicalJoin, Operand};
use crate::{err_system::err_types::ErrTypes, tok_system::tokens::Token};
use std::collections::HashMap;

// Pre-computed valid type combinations using phf for O(1) lookup
static VALID_TYPE_COMBINATIONS: phf::Map<&'static str, bool> = phf::phf_map! {
    "i8_i8" => true, "i8_i16" => true, "i8_i32" => true,
    "i8_i64" => true, "i8_f32" => true, "i8_f64" => true,
    "i16_i8" => true, "i16_i16" => true, "i16_i32" => true,
    "i16_i64" => true, "i16_f32" => true, "i16_f64" => true,
    "i32_i8" => true, "i32_i16" => true, "i32_i32" => true,
    "i32_i64" => true, "i32_f32" => true, "i32_f64" => true,
    "i64_i8" => true, "i64_i16" => true, "i64_i32" => true,
    "i64_i64" => true, "i64_f32" => true, "i64_f64" => true,
    "f32_i8" => true, "f32_i16" => true, "f32_i32" => true,
    "f32_i64" => true, "f32_f32" => true, "f32_f64" => true,
    "f64_i8" => true, "f64_i16" => true, "f64_i32" => true,
    "f64_i64" => true, "f64_f32" => true, "f64_f64" => true,
    "str_str" => true, "ch_ch" => true,
};

#[inline(always)]
pub fn parse_condition(
    raw_cond: &[Token],
    collected_errors: &mut Vec<ErrTypes>,
    collected_vars: &[(String, &'static str)],
    line: i32,
) -> Condition {
    // Pre-allocate with estimated capacity
    let mut child_conditions = Vec::with_capacity(raw_cond.len() / 4);
    let mut tokens = raw_cond.iter().peekable();

    // Create variable type lookup table
    let var_types: HashMap<&str, &'static str> = collected_vars
        .iter()
        .map(|(name, typ)| (name.as_str(), *typ))
        .collect();

    #[inline(always)]
    fn parse_operand(
        s: &str,
        var_types: &HashMap<&str, &'static str>,
        collected_errors: &mut Vec<ErrTypes>,
        line: i32,
    ) -> (Operand, &'static str) {
        if s.starts_with('"') && s.ends_with('"') {
            (Operand::Literal(s[1..s.len() - 1].to_string()), "str")
        } else if let Ok(n) = s.parse::<f64>() {
            (Operand::Numeric(n), "f64")
        } else if let Some(&var_type) = var_types.get(s) {
            (Operand::Variable(s.to_string()), var_type)
        } else {
            collected_errors.push(ErrTypes::VarNotFound(line));
            (Operand::Variable(s.to_string()), "unknown")
        }
    }

    while let Some(tok) = tokens.peek() {
        if matches!(tok, Token::Space) {
            tokens.next();
            continue;
        }

        // Parse left operand
        let (left_operand, left_type) = match tokens.next() {
            Some(Token::Iden(s)) => parse_operand(s, &var_types, collected_errors, line),
            _ => continue,
        };

        // Skip spaces efficiently
        while matches!(tokens.peek(), Some(Token::Space)) {
            tokens.next();
        }

        // Parse operator with early return
        let operator = match tokens.next() {
            Some(Token::DoubleEqSign) | Some(Token::EqSign) => CondToks::Equal,
            Some(Token::GreaterThan) => CondToks::GreaterThan,
            Some(Token::LessThan) => CondToks::LessThan,
            Some(Token::Not) if matches!(tokens.peek(), Some(Token::EqSign)) => {
                tokens.next();
                CondToks::NotEqual
            }
            _ => {
                collected_errors.push(ErrTypes::MissingOperator(line));
                continue;
            }
        };

        // Skip spaces efficiently
        while matches!(tokens.peek(), Some(Token::Space)) {
            tokens.next();
        }

        // Parse right operand
        let (right_operand, right_type) = match tokens.next() {
            Some(Token::Iden(s)) => parse_operand(s, &var_types, collected_errors, line),
            _ => continue,
        };

        // Check type compatibility using O(1) lookup
        if !VALID_TYPE_COMBINATIONS.contains_key(&format!("{}_{}", left_type, right_type)) {
            collected_errors.push(ErrTypes::TypeMismatch(line));
            continue;
        }

        // Skip spaces efficiently
        while matches!(tokens.peek(), Some(Token::Space)) {
            tokens.next();
        }

        // Parse logical joiner
        let joiner = match tokens.peek() {
            Some(Token::And) => {
                tokens.next();
                Some(LogicalJoin::And)
            }
            Some(Token::Or) => {
                tokens.next();
                Some(LogicalJoin::Or)
            }
            _ => None,
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
