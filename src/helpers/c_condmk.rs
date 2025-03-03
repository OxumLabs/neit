use crate::{
    err_system::err_types::ErrTypes,
    helpers::{CondToks, Condition, LogicalJoin, Operand},
    parse_systems::{COLLECTED_ERRORS, COLLECTED_VARS, LINE},
};

pub fn mk_c_cond(cond: &Condition) -> String {
    let mut result = String::new();
    let is_numeric_literal = |lit: &str| lit.parse::<f64>().is_ok();
    for (i, child) in cond.child_conditions.iter().enumerate() {
        let vars = COLLECTED_VARS.lock().unwrap();
        let op_str = match child.operator {
            CondToks::Equal => "==",
            CondToks::NotEqual => "!=",
            CondToks::GreaterThan => ">",
            CondToks::LessThan => "<",
            CondToks::GreaterThanOrEqual => ">=",
            CondToks::LessThanOrEqual => "<=",
        };
        let snippet = match (&child.left, &child.right) {
            (Operand::Variable(var_name), Operand::Literal(lit)) => {
                if is_numeric_literal(lit) {
                    format!("({var} {op} {lit})", var = var_name, op = op_str, lit = lit)
                } else {
                    let var_type = vars.iter().find(|(name, _)| name == var_name).map(|(_, ty)| *ty);
                    match var_type {
                        Some("Str") => format!("(strcmp({var}.str, \"{lit}\") {op} 0)", var = var_name, lit = lit, op = op_str),
                        Some("char") => format!("(strcmp({var}, \"{lit}\") {op} 0)", var = var_name, lit = lit, op = op_str),
                        Some("I32") | Some("I8") | Some("I16") | Some("I64") | Some("F32") | Some("F64") => {
                            if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                                unsafe { errors.push(ErrTypes::TypeMismatch(LINE)) };
                            }
                            "0".to_string()
                        },
                        _ => format!("(strcmp({var}.str, \"{lit}\") {op} 0)", var = var_name, lit = lit, op = op_str),
                    }
                }
            }
            (Operand::Literal(lit), Operand::Variable(var_name)) => {
                if is_numeric_literal(lit) {
                    format!("({lit} {op} {var})", lit = lit, op = op_str, var = var_name)
                } else {
                    let var_type = vars.iter().find(|(name, _)| name == var_name).map(|(_, ty)| *ty);
                    match var_type {
                        Some("Char") | Some("Str") => format!("(strcmp(\"{lit}\", {var}.str) {op} 0)", lit = lit, var = var_name, op = op_str),
                        Some("I32") | Some("I8") | Some("I16") | Some("I64") | Some("F32") | Some("F64") => {
                            if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                                unsafe { errors.push(ErrTypes::TypeMismatch(LINE)) };
                            }
                            "0".to_string()
                        },
                        _ => format!("(strcmp(\"{lit}\", {var}.str) {op} 0)", lit = lit, var = var_name, op = op_str),
                    }
                }
            }
            (Operand::Variable(v1), Operand::Variable(v2)) => {
                let var_type = vars.iter().find(|(name, _)| name == v1).map(|(_, ty)| *ty);
                if let Some(t) = var_type {
                    if t == "Char" || t == "Str" {
                        format!("(strcmp({v1}.str, {v2}.str) {op} 0)", v1 = v1, v2 = v2, op = op_str)
                    } else if t == "I32" || t == "I8" || t == "I16" || t == "I64" || t == "F32" || t == "F64" {
                        format!("({v1} {op} {v2})", v1 = v1, op = op_str, v2 = v2)
                    } else {
                        format!("({v1} {op} {v2})", v1 = v1, op = op_str, v2 = v2)
                    }
                } else {
                    format!("({v1} {op} {v2})", v1 = v1, op = op_str, v2 = v2)
                }
            }
            (Operand::Variable(var_name), Operand::Numeric(num)) => {
                let var_type = vars.iter().find(|(name, _)| name == var_name).map(|(_, ty)| *ty);
                match var_type {
                    Some("I32") | Some("I8") | Some("I16") | Some("I64") | Some("F32") | Some("F64") =>
                        format!("({var} {op} {num})", var = var_name, op = op_str, num = num),
                    Some("Char") | Some("Str") => {
                        if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                            unsafe { errors.push(ErrTypes::TypeMismatch(LINE)) };
                        }
                        "0".to_string()
                    }
                    _ => format!("({var} {op} {num})", var = var_name, op = op_str, num = num),
                }
            }
            (Operand::Numeric(num), Operand::Variable(var_name)) => {
                let var_type = vars.iter().find(|(name, _)| name == var_name).map(|(_, ty)| *ty);
                match var_type {
                    Some("I32") | Some("I8") | Some("I16") | Some("I64") | Some("F32") | Some("F64") =>
                        format!("({num} {op} {var})", num = num, op = op_str, var = var_name),
                    Some("Char") | Some("Str") => {
                        if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                            unsafe { errors.push(ErrTypes::TypeMismatch(LINE)) };
                        }
                        "0".to_string()
                    }
                    _ => format!("({num} {op} {var})", num = num, op = op_str, var = var_name),
                }
            }
            (Operand::Numeric(n1), Operand::Numeric(n2)) => {
                format!("({n1} {op} {n2})", n1 = n1, op = op_str, n2 = n2)
            }
            (Operand::Literal(lit1), Operand::Literal(lit2)) => {
                if is_numeric_literal(lit1) && is_numeric_literal(lit2) {
                    format!("({lit1} {op} {lit2})", lit1 = lit1, op = op_str, lit2 = lit2)
                } else {
                    format!("(strcmp(\"{lit1}\", \"{lit2}\") {op} 0)", lit1 = lit1, lit2 = lit2, op = op_str)
                }
            }
            (Operand::Numeric(num), Operand::Literal(lit)) => {
                if is_numeric_literal(lit) {
                    format!("({num} {op} {lit})", num = num, op = op_str, lit = lit)
                } else {
                    if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                        unsafe { errors.push(ErrTypes::TypeMismatch(LINE)) };
                    }
                    "0".to_string()
                }
            }
            (Operand::Literal(lit), Operand::Numeric(num)) => {
                if is_numeric_literal(lit) {
                    format!("({lit} {op} {num})", lit = lit, op = op_str, num = num)
                } else {
                    if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                        unsafe { errors.push(ErrTypes::TypeMismatch(LINE)) };
                    }
                    "0".to_string()
                }
            }
        };
        result.push_str(&snippet);
        if let Some(joiner) = &child.joiner {
            let join_str = match joiner {
                LogicalJoin::And => " && ",
                LogicalJoin::Or => " || ",
            };
            result.push_str(join_str);
        } else if i < cond.child_conditions.len() - 1 {
            result.push_str(" && ");
        }
    }
    result
}
