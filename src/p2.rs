use crate::{
    err::ErrT,
    lex::{TokType, Tokens},
    p::{parse, VVal, NST},
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub left: String,
    pub operator: String,
    pub right: String,
    pub left_type: ValueType,
    pub right_type: ValueType,
    pub c_operator: String,
    pub c_code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Int,
    Float,
    Str,
    Bool,
}

pub fn p2(
    toks: &Tokens,
    tok_iter: &mut std::iter::Peekable<std::slice::Iter<'_, Tokens>>,
    codes: &[&str],
    errors: &mut Vec<ErrT>,
    nst: &mut Vec<NST>,
    ln: &mut usize,
    vars: &HashMap<String, VVal>,
    file: &str,
) -> bool {
    println!("toks in p2 : {:?}",toks);
    match (toks.get_type(), toks.get_value()) {
        (TokType::CMD, "if") => {
            let mut cond = String::new();
            let mut body_tokens = Vec::new();
            let mut in_parentheses = false;
            let mut brace_count = 0;

            // Parse condition inside parentheses
            while let Some(tok) = tok_iter.next() {
                match (tok.get_type(), tok.get_value()) {
                    (TokType::OP, "(") if !in_parentheses => {
                        in_parentheses = true;
                    }
                    (TokType::EOL, _) => {
                        *ln += 1;
                    }
                    (TokType::OP, ")") if in_parentheses => {
                        in_parentheses = false;
                        break; // Exit condition parsing
                    }
                    (TokType::SPACE, _) => {
                        continue; // Ignore spaces inside condition
                    }
                    (_, _) if in_parentheses => {
                        cond.push_str(tok.get_value());
                    }
                    _ => {
                        
                        errors.push(ErrT::UnmatchedParen(*ln, codes[*ln].to_string()));
                        return true;
                    }
                }
            }

            // Check for unmatched or empty condition
            if in_parentheses {
                
                errors.push(ErrT::UnmatchedParen(*ln, codes[*ln].to_string()));
                return true;
            }
            if cond.is_empty() {
                errors.push(ErrT::EmptyCond(*ln, codes[*ln].to_string()));
                return true;
            }

            // Parse the condition
            let cond_parsed = parse_condition(&cond, *ln, errors, vars, nst);
            let condition = match cond_parsed {
                Some(cond) => cond,
                None => {
                    errors.push(ErrT::InVCond(*ln, cond.clone()));
                    return true;
                }
            };

            // Parse body inside braces with brace counting
            while let Some(tok) = tok_iter.next() {
                match (tok.get_type(), tok.get_value()) {
                    (TokType::OP, "{") => {
                        brace_count += 1; // Increment brace count
                        if brace_count == 1 {
                            continue; // Skip the first `{` to start body parsing
                        }
                    }
                    (TokType::OP, "}") => {
                        brace_count -= 1; // Decrement brace count
                        if brace_count == 0 {
                            break; // Exit body parsing
                        }
                    }
                    (_, _) if brace_count > 0 => {
                        body_tokens.push(tok.clone());
                    }
                    _ => {}
                }
            }

            // Check for unmatched braces
            if brace_count != 0 {
                errors.push(ErrT::UnmatchedParen(
                    *ln,
                    "Unmatched braces in while loop".to_string(),
                ));
                return true;
            }

            // Check for empty body
            if body_tokens.is_empty() {
                errors.push(ErrT::InVCond(*ln, "Empty body for while loop".to_string()));
                return true;
            }

            // Parse the body tokens
            let body = parse(&body_tokens, codes, file, false, errors);
            nst.push(NST::NIF(condition, body));
            return true;
        }
        #[allow(unused)]
        (TokType::CMD, v) if vars.contains_key(v) => {
            let mut is_var_declared = false;
            let mut collected_value = String::new();
        
            while let Some(tok) = tok_iter.next() {
                match tok.get_type() {
                    TokType::EOL => {
                        if is_var_declared {
                            let var_name = v;
                            let var_value = collected_value.trim().to_string();
        
                            if var_value == "takein()" {
                                nst.push(NST::VRDInput(var_name.to_string()));
                                return true;
                            }
        
                            if vars.contains_key(var_name) {
                                let mut is_valid_value = false;
                                let resolved_value = if var_value.starts_with('"') && var_value.ends_with('"') {
                                    is_valid_value = true;
                                    VVal::Str(var_value[1..var_value.len() - 1].to_string())
                                } else if var_value.parse::<i32>().is_ok() {
                                    is_valid_value = true;
                                    VVal::Int(var_value.parse::<i32>().unwrap())
                                } else if var_value.parse::<f32>().is_ok() {
                                    is_valid_value = true;
                                    VVal::F(var_value.parse::<f32>().unwrap())
                                } else if let Some(existing_var) = vars.get(&var_value) {
                                    is_valid_value = true;
        
                                    match existing_var {
                                        VVal::Str(_) => VVal::VarRef(var_name.to_string(), "s".to_string()),
                                        VVal::Int(_) => VVal::VarRef(var_name.to_string(), "i".to_string()),
                                        VVal::F(_) => VVal::VarRef(var_name.to_string(), "f".to_string()),
                                        _ => {
                                            VVal::VarRef(var_name.to_string(), match existing_var{
                                                VVal::Str(_) => "s".to_string(),
                                                VVal::Int(_) => "i".to_string(),
                                                VVal::F(_) => "f".to_string(),
                                                _ => "v".to_string(),
                                            })
                                        }
                                    }
                                } else {
                                    errors.push(ErrT::InValidVarVal(*ln, var_value.clone()));
                                    VVal::Str(var_value.clone())
                                };
        
                                if let Some(original_var) = vars.get(var_name) {
                                    match (original_var, &resolved_value) {
                                        (VVal::Str(_), VVal::Str(_)) |
                                        (VVal::Int(_), VVal::Int(_)) |
                                        (VVal::F(_), VVal::F(_)) |
                                        (VVal::VarRef(_, _), VVal::VarRef(_, _)) => {
                                            nst.push(NST::VarRD(var_name.to_string(), resolved_value));
                                            return true;
                                        }
                                        _ => {
                                            errors.push(ErrT::InValidVarVal(*ln, var_value.clone()));
                                            return true;
                                        }
                                    }
                                } else {
                                    errors.push(ErrT::InValidVarVal(*ln, var_value.clone()));
                                    break;
                                }
                            } else {
                                errors.push(ErrT::InValidVarVal(*ln, var_value.clone()));
                                break;
                            }
                        }
                        break;
                    }
                    TokType::OP if tok.get_value() == "=" => {
                        is_var_declared = true;
                    }
                    _ if is_var_declared => {
                        collected_value.push_str(tok.get_value());
                    }
                    _ => {}
                }
            }
        
            *ln += 1;
            true
        }
        
        _ => {
            return false;
        }
    }
}
#[allow(unused)]
pub fn parse_condition(
    condition: &str,
    line_number: usize,
    errors: &mut Vec<ErrT>,
    vars: &HashMap<String, VVal>,
    nst: &Vec<NST>,
) -> Option<Condition> {
    let mut index = 0;
    let mut operand_stack = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();
    let condition = condition.replace(" ", "");

    while index < condition.len() {
        let c = condition.chars().nth(index).unwrap();
        match c {
            '(' => {
                index += 1;
                if let Some(nested_condition) =
                    parse_condition(&condition[index..], line_number, errors, vars, nst)
                {
                    operand_stack.push(format!("({})", nested_condition.c_code));
                }
            }
            ')' => {
                index += 1;
                break;
            }
            '0'..='9' | '"' | '\'' | 'a'..='z' | 'A'..='Z' | '_' => {
                if let Some(operand) = parse_operand_char_by_char(
                    &condition,
                    &mut index,
                    line_number,
                    errors,
                    vars,
                    nst,
                ) {
                    operand_stack.push(operand);
                }
            }
            '&' | '|' | '=' | '!' | '<' | '>' | '+' | '-' | '*' | '/' => {
                if let Some(operator) =
                    parse_operator_char_by_char(&condition, &mut index, line_number, errors)
                {
                    while let Some(top_operator) = operator_stack.last() {
                        if has_higher_precedence(top_operator, &operator) {
                            apply_operator(
                                &mut operand_stack,
                                operator_stack.pop().unwrap(),
                                line_number,
                                errors,
                                vars,
                            )?;
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(operator);
                }
            }
            _ => {
                errors.push(ErrT::InvalidCondOp(
                    line_number,
                    format!("Unexpected character: `{}`", c),
                ));
                return None;
            }
        }
    }

    while let Some(operator) = operator_stack.pop() {
        apply_operator(&mut operand_stack, operator, line_number, errors, vars)?;
    }

    if operand_stack.len() == 1 {
        let c_code = operand_stack.pop().unwrap();
        return Some(Condition {
            left: String::new(),
            operator: String::new(),
            right: String::new(),
            left_type: ValueType::Bool,
            right_type: ValueType::Bool,
            c_operator: String::new(),
            c_code,
        });
    }

    errors.push(ErrT::InvalidCondOp(
        line_number,
        "Invalid condition structure.".to_string(),
    ));
    None
}
fn parse_operand_char_by_char(
    condition: &str,
    index: &mut usize,
    line_number: usize,
    errors: &mut Vec<ErrT>,
    vars: &HashMap<String, VVal>,
    nst: &Vec<NST>,
) -> Option<String> {
    let mut buffer = String::new();
    while *index < condition.len() {
        let c = condition.chars().nth(*index).unwrap();
        match c {
            ' ' | '(' | ')' | '&' | '|' | '=' | '!' | '<' | '>' | '+' | '-' | '*' | '/' => break,
            _ => {
                buffer.push(c);
                *index += 1;
            }
        }
    }

    let value_type = determine_value_type(&buffer, vars, errors, line_number, nst)?;
    match value_type {
        ValueType::Str | ValueType::Int | ValueType::Float => Some(buffer),
        _ => {
            errors.push(ErrT::InVCond(
                line_number,
                format!("Invalid operand: {}", buffer),
            ));
            None
        }
    }
}

fn parse_operator_char_by_char(
    condition: &str,
    index: &mut usize,
    line_number: usize,
    errors: &mut Vec<ErrT>,
) -> Option<String> {
    let mut operator = String::new();
    while *index < condition.len() {
        let c = condition.chars().nth(*index).unwrap();
        match c {
            '=' | '!' | '<' | '>' | '&' | '|' => {
                operator.push(c);
                *index += 1;
            }
            _ => break,
        }
    }

    if ["==", "!=", "<", ">", "<=", ">=", "&&", "||"].contains(&operator.as_str()) {
        Some(operator)
    } else {
        errors.push(ErrT::InvalidCondOp(
            line_number,
            format!("Invalid operator: `{}`", operator),
        ));
        None
    }
}

fn has_higher_precedence(op1: &str, op2: &str) -> bool {
    let precedence = |op: &str| match op {
        "&&" | "||" => 1,
        "<" | ">" | "<=" | ">=" => 2,
        "==" | "!=" => 3,
        "+" | "-" => 4,
        "*" | "/" => 5,
        _ => 0,
    };

    precedence(op1) > precedence(op2)
}
fn apply_operator(
    operand_stack: &mut Vec<String>,
    operator: String,
    line_number: usize,
    errors: &mut Vec<ErrT>,
    vars: &HashMap<String, VVal>,
) -> Option<()> {
    if operand_stack.len() < 2 {
        errors.push(ErrT::InvalidCondOp(
            line_number,
            "Not enough operands for operator.".to_string(),
        ));
        return None;
    }

    let right = operand_stack.pop().unwrap();
    let left = operand_stack.pop().unwrap();

    let combined = if (operator == "==" || operator == "!=")
        && (
            (left.starts_with('"') && left.ends_with('"')) // String literal check for left operand
            || (right.starts_with('"') && right.ends_with('"')) // String literal check for right operand
            || vars.get(&left).map_or(false, |v| matches!(v, VVal::Str(_))) // Check if left is a variable of type Str
            || vars.get(&right).map_or(false, |v| matches!(v, VVal::Str(_))) // Check if right is a variable of type Str
        )
    {
        let left = if left.starts_with('"') && left.ends_with('"') || vars.get(&left).map_or(false, |v| matches!(v, VVal::Str(_))) {
            format!("to_nstr({})", left)
        } else {
            left
        };

        let right = if right.starts_with('"') && right.ends_with('"') || vars.get(&right).map_or(false, |v| matches!(v, VVal::Str(_))) {
            format!("to_nstr({})", right)
        } else {
            right
        };

        format!(
            "nstr_cmp({}, {}) {} 0",
            left,
            right,
            if operator == "==" { "==" } else { "!=" }
        )
    } else {
        format!("{} {} {}", left, operator, right)
    };

    operand_stack.push(combined);
    Some(())
}

fn determine_value_type(
    expr: &str,
    vars: &HashMap<String, VVal>,
    errors: &mut Vec<ErrT>,
    line_number: usize,
    nst: &Vec<NST>,
) -> Option<ValueType> {
    if expr.starts_with('"') || expr.starts_with('\'') {
        return Some(ValueType::Str);
    }

    match vars.get(expr) {
        Some(VVal::Int(_)) => Some(ValueType::Int),
        Some(VVal::F(_)) => Some(ValueType::Float),
        Some(VVal::Str(_)) => Some(ValueType::Str),
        _ => {
            if expr.parse::<i32>().is_ok() {
                Some(ValueType::Int)
            } else if expr.parse::<f32>().is_ok() {
                Some(ValueType::Float)
            } else {
                for i in nst {
                    if let NST::Input(n) = i {
                        if n == expr {
                            return Some(ValueType::Str);
                        }
                    }
                }
                errors.push(ErrT::VNF(line_number, expr.to_string()));
                None
            }
        }
    }
}
