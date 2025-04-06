use crate::{
    err_system::err_types::ErrTypes,
    helpers::{CondToks, Condition, LogicalJoin, Operand},
};
use std::collections::{HashMap, HashSet};

// Pre-computed operators for O(1) lookup
use lazy_static::lazy_static;

lazy_static! {
    static ref OPERATORS: HashMap<CondToks, &'static str> = {
        let mut m = HashMap::new();
        m.insert(CondToks::Equal, "==");
        m.insert(CondToks::NotEqual, "!=");
        m.insert(CondToks::GreaterThan, ">");
        m.insert(CondToks::LessThan, "<");
        m.insert(CondToks::GreaterThanOrEqual, ">=");
        m.insert(CondToks::LessThanOrEqual, "<=");
        m
    };

    // Pre-computed numeric types for O(1) lookup
    static ref NUMERIC_TYPES: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("I32");
        s.insert("I8");
        s.insert("I16");
        s.insert("I64");
        s.insert("F32");
        s.insert("F64");
        s
    };
}

#[inline(always)]
pub fn mk_c_cond(
    cond: &Condition,
    collected_errors: &mut Vec<ErrTypes>,
    collected_vars: &[(String, &'static str)],
    line: i32,
) -> String {
    // Pre-allocate with estimated capacity
    let mut result = String::with_capacity(cond.child_conditions.len() * 32);

    // Create variable type lookup table
    let var_types: HashMap<&str, &'static str> = collected_vars
        .iter()
        .map(|(name, typ)| (name.as_str(), *typ))
        .collect();

    #[inline(always)]
    fn is_numeric_literal(lit: &str) -> bool {
        lit.bytes()
            .all(|b| b.is_ascii_digit() || b == b'.' || b == b'-')
    }

    for (i, child) in cond.child_conditions.iter().enumerate() {
        let op_str = OPERATORS[&child.operator];

        let snippet = match (&child.left, &child.right) {
            (Operand::Variable(var_name), Operand::Literal(lit)) => {
                handle_var_lit(var_name, lit, op_str, &var_types, collected_errors, line)
            }
            (Operand::Literal(lit), Operand::Variable(var_name)) => {
                handle_lit_var(lit, var_name, op_str, &var_types, collected_errors, line)
            }
            (Operand::Variable(v1), Operand::Variable(v2)) => {
                handle_var_var(v1, v2, op_str, &var_types)
            }
            (Operand::Variable(var_name), Operand::Numeric(num))
            | (Operand::Numeric(num), Operand::Variable(var_name)) => {
                handle_var_num(var_name, *num, op_str, &var_types, collected_errors, line)
            }
            (Operand::Numeric(n1), Operand::Numeric(n2)) => {
                format!("({n1} {op_str} {n2})")
            }
            (Operand::Literal(lit1), Operand::Literal(lit2)) => {
                if is_numeric_literal(lit1) && is_numeric_literal(lit2) {
                    format!("({lit1} {op_str} {lit2})")
                } else {
                    format!("(strcmp(\"{lit1}\", \"{lit2}\") {op_str} 0)")
                }
            }
            (left, right) => handle_mixed_operands(left, right, op_str, collected_errors, line),
        };

        result.push_str(&snippet);

        if i < cond.child_conditions.len() - 1 {
            result.push_str(if let Some(joiner) = &child.joiner {
                match joiner {
                    LogicalJoin::And => " && ",
                    LogicalJoin::Or => " || ",
                }
            } else {
                " && "
            });
        }
    }

    result
}

#[inline(always)]
fn handle_var_lit(
    var_name: &str,
    lit: &str,
    op_str: &str,
    var_types: &HashMap<&str, &'static str>,
    collected_errors: &mut Vec<ErrTypes>,
    line: i32,
) -> String {
    if lit
        .bytes()
        .all(|b| b.is_ascii_digit() || b == b'.' || b == b'-')
    {
        format!("({var_name} {op_str} {lit})")
    } else {
        match var_types.get(var_name) {
            Some(&"Str") | Some(&"char") => {
                format!("(strcmp({var_name}.str, \"{lit}\") {op_str} 0)")
            }
            Some(&typ) if NUMERIC_TYPES.contains(typ) => {
                collected_errors.push(ErrTypes::TypeMismatch(line));
                String::from("0")
            }
            _ => format!("(strcmp({var_name}.str, \"{lit}\") {op_str} 0)"),
        }
    }
}

#[inline(always)]
fn handle_lit_var(
    lit: &str,
    var_name: &str,
    op_str: &str,
    var_types: &HashMap<&str, &'static str>,
    collected_errors: &mut Vec<ErrTypes>,
    line: i32,
) -> String {
    if lit
        .bytes()
        .all(|b| b.is_ascii_digit() || b == b'.' || b == b'-')
    {
        format!("({lit} {op_str} {var_name})")
    } else {
        match var_types.get(var_name) {
            Some(&"Str") | Some(&"Char") => {
                format!("(strcmp(\"{lit}\", {var_name}.str) {op_str} 0)")
            }
            Some(&typ) if NUMERIC_TYPES.contains(typ) => {
                collected_errors.push(ErrTypes::TypeMismatch(line));
                String::from("0")
            }
            _ => format!("(strcmp(\"{lit}\", {var_name}.str) {op_str} 0)"),
        }
    }
}

#[inline(always)]
fn handle_var_var(
    v1: &str,
    v2: &str,
    op_str: &str,
    var_types: &HashMap<&str, &'static str>,
) -> String {
    match var_types.get(v1) {
        Some(&"Char") | Some(&"Str") => format!("(strcmp({v1}.str, {v2}.str) {op_str} 0)"),
        Some(&typ) if NUMERIC_TYPES.contains(typ) => format!("({v1} {op_str} {v2})"),
        _ => format!("({v1} {op_str} {v2})"),
    }
}

#[inline(always)]
fn handle_var_num(
    var_name: &str,
    num: f64,
    op_str: &str,
    var_types: &HashMap<&str, &'static str>,
    collected_errors: &mut Vec<ErrTypes>,
    line: i32,
) -> String {
    match var_types.get(var_name) {
        Some(&typ) if NUMERIC_TYPES.contains(typ) => format!("({var_name} {op_str} {num})"),
        Some(&"Char") | Some(&"Str") => {
            collected_errors.push(ErrTypes::TypeMismatch(line));
            String::from("0")
        }
        _ => format!("({var_name} {op_str} {num})"),
    }
}

#[inline(always)]
fn handle_mixed_operands(
    left: &Operand,
    right: &Operand,
    op_str: &str,
    collected_errors: &mut Vec<ErrTypes>,
    line: i32,
) -> String {
    match (left, right) {
        (Operand::Numeric(num), Operand::Literal(lit))
        | (Operand::Literal(lit), Operand::Numeric(num)) => {
            if lit
                .bytes()
                .all(|b| b.is_ascii_digit() || b == b'.' || b == b'-')
            {
                format!("({num} {op_str} {lit})")
            } else {
                collected_errors.push(ErrTypes::TypeMismatch(line));
                String::from("0")
            }
        }
        _ => String::from("0"),
    }
}
