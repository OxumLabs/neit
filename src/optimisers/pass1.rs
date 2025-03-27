use crate::{
    helpers::{Condition, Operand},
    parse_systems::{AST, PrintTokTypes, Variables},
};
use std::collections::HashSet;

pub fn pass1(ast: &mut Vec<AST>) {
    let mut used_vars = HashSet::new();

    #[inline(always)]
    fn collect_usage_from_str(s: &str, used: &mut HashSet<String>) {
        s.split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|token| !token.is_empty() && token.chars().any(|c| c.is_alphabetic()))
            .for_each(|token| {
                used.insert(token.to_string());
            });
    }

    #[inline(always)]
    fn collect_usage_condition(cond: &Condition, used: &mut HashSet<String>) {
        // Instead of converting the condition to a debug string, iterate directly over its child conditions.
        for child in &cond.child_conditions {
            if let Operand::Variable(var) = &child.left {
                used.insert(var.clone());
            }
            if let Operand::Variable(var) = &child.right {
                used.insert(var.clone());
            }
        }
    }

    fn collect_usage_ast(ast: &Vec<AST>, used: &mut HashSet<String>) {
        for node in ast {
            match node {
                AST::Print { descriptor: _, text } => {
                    for ptok in text {
                        if let PrintTokTypes::Var(v) = ptok {
                            used.insert(v.clone());
                        }
                    }
                }
                AST::VarAssign(var) => {
                    if let Variables::MATH(_, expr) = var {
                        collect_usage_from_str(expr, used);
                    }
                }
                AST::While(body, cond) | AST::IF(body, cond) => {
                    collect_usage_condition(cond, used);
                    collect_usage_ast(body, used);
                }
                _ => {}
            }
        }
    }

    collect_usage_ast(ast, &mut used_vars);

    ast.retain(|node| match node {
        AST::Var(var) => {
            let name = match var {
                Variables::MATH(n, _) => n.to_string(),
                Variables::Char(n, _) => n.to_string(),
                Variables::I8(n, _) => n.to_string(),
                Variables::I16(n, _) => n.to_string(),
                Variables::I32(n, _) => n.to_string(),
                Variables::I64(n, _) => n.to_string(),
                Variables::F32(n, _) => n.to_string(),
                Variables::F64(n, _) => n.to_string(),
                Variables::Str(n, _) => n.to_string(),
                Variables::REF(n, _) => n.to_string(),
            };
            used_vars.contains(&name)
        }
        _ => true,
    });
}
