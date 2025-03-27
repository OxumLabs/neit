use crate::{
    helpers::c_condmk::mk_c_cond,
    parse_systems::{PrintTokTypes, Variables, AST},
    err_system::err_types::ErrTypes,
};
use std::fmt::Write;
use std::collections::HashMap;

fn contains_letters(s: &str) -> bool {
    s.chars().any(|c| c.is_alphabetic())
}

fn evaluate_numeric_expr(expr: &str) -> Option<f64> {
    let expr = expr.replace(" ", "");
    let expr = expr.replace("-", "+-");
    let tokens: Vec<&str> = expr.split('+').filter(|s| !s.is_empty()).collect();
    let mut result = 0.0;
    for token in tokens {
        if let Ok(val) = token.parse::<f64>() {
            result += val;
        } else {
            return None;
        }
    }
    Some(result)
}

fn eval_math_expr(expr: &str, math_values: &HashMap<String, f64>) -> Option<f64> {
    if !contains_letters(expr) {
        return evaluate_numeric_expr(expr);
    }
    let mut substituted = expr.to_string();
    for token in expr.split(|c: char| !c.is_alphanumeric() && c != '_') {
        if !token.is_empty() && token.chars().any(|c| c.is_alphabetic()) {
            if let Some(value) = math_values.get(token) {
                substituted = substituted.replace(token, &value.to_string());
            } else {
                return None;
            }
        }
    }
    if !contains_letters(&substituted) {
        evaluate_numeric_expr(&substituted)
    } else {
        None
    }
}

#[allow(non_snake_case)]
pub fn make_c(
    ast: &[AST],
    gen_main_function: bool,
    collected_vars: &mut Vec<(String, &'static str)>,
    collected_errors: &mut Vec<ErrTypes>,
    math_values: &mut HashMap<String, f64>,
) -> String {
    let mut code = String::with_capacity(4096);
    let binding = collected_vars.clone();
    let var_types: HashMap<&str, &'static str> = binding.iter().map(|(name, typ)| (name.as_str(), *typ)).collect();
    const HEADER: &str = "#include \"nulibc.h\"\n#include <stdio.h>\n#include <stdlib.h>\nint main(){\n";
    
    if gen_main_function {
        code.push_str(HEADER);
    }
    
    static FORMAT_SPECIFIERS: [(&str, &str); 6] = [
        ("ch", "%c"),
        ("i8", "%d"),
        ("i16", "%d"),
        ("i32", "%d"),
        ("i64", "%d"),
        ("f32", "%f"),
    ];
    
    let format_map: HashMap<&str, &str> = FORMAT_SPECIFIERS.iter().copied().collect();

    for node in ast {
        match node {
            AST::Print { descriptor: fd, text } => {
                let mut fmt = String::with_capacity(text.len() * 2);
                let mut args = Vec::new();
                for ptok in text {
                    match ptok {
                        PrintTokTypes::Newline => fmt.push_str("\\n"),
                        PrintTokTypes::Space => fmt.push(' '),
                        PrintTokTypes::Word(w) => fmt.push_str(w),
                        PrintTokTypes::Var(v) => {
                            if let Some(&typ) = var_types.get(v.as_str()) {
                                if let Some(fmt_spec) = format_map.get(typ) {
                                    fmt.push_str(fmt_spec);
                                }
                            }
                            args.push(v.as_str());
                        }
                    }
                }
                if !args.is_empty() {
                    write!(&mut code, "nprintf({},\"{}\",{});\n", fd.display(), fmt, args.join(",")).unwrap();
                } else {
                    write!(&mut code, "nprintf({},\"{}\");\n", fd.display(), fmt).unwrap();
                }
            }
            AST::Var(var) => {
                use Variables::*;
                match var {
                    MATH(n, v) => {
                        let computed_value = eval_math_expr(v, math_values);
                        let determined_type = if let Some(result) = computed_value {
                            if result.fract() == 0.0 {
                                if result >= i32::MIN as f64 && result <= i32::MAX as f64 {
                                    "i32"
                                } else {
                                    "i64"
                                }
                            } else {
                                let f32_val = result as f32;
                                if (f32_val as f64) != result {
                                    "f64"
                                } else {
                                    "f32"
                                }
                            }
                        } else {
                            if v.contains('.') {
                                "f32"
                            } else {
                                "i32"
                            }
                        };
                        collected_vars.push((n.clone(), determined_type));
                        if let Some(result) = computed_value {
                            math_values.insert(n.clone(), result);
                        }
                        if determined_type == "i32" || determined_type == "i64" {
                            if let Some(result) = computed_value {
                                write!(&mut code, "{} {} = {};\n", determined_type, n, result as i64).unwrap();
                            } else {
                                let v_clean = if v.ends_with(".0") { &v[..v.len()-2] } else { v };
                                write!(&mut code, "{} {} = {};\n", determined_type, n, v_clean).unwrap();
                            }
                        } else {
                            write!(&mut code, "{} {} = {};\n", determined_type, n, v).unwrap();
                        }
                    }
                    Char(n, v) => write!(&mut code, "char {} = '{}';\n", n, v).unwrap(),
                    I8(n, v) => write!(&mut code, "i8 {} = {};\n", n, v).unwrap(),
                    I16(n, v) => write!(&mut code, "i16 {} = {};\n", n, v).unwrap(),
                    I32(n, v) => write!(&mut code, "i32 {} = {};\n", n, v).unwrap(),
                    I64(n, v) => write!(&mut code, "i64 {} = {};\n", n, v).unwrap(),
                    F32(n, v) => write!(&mut code, "f32 {} = {};\n", n, v).unwrap(),
                    F64(n, v) => write!(&mut code, "double {} = {};\n", n, v).unwrap(),
                    Str(n, v) => write!(&mut code, "nstring {} = nstr_new(\"{}\");\n", n, v).unwrap(),
                    REF(n, v) => {
                        let (typ, actual) = resolve_ref(ast, v);
                        write!(&mut code, "{} {} = {};\n", typ, n, actual).unwrap();
                    }
                }
            }
            AST::While(body, cond) => {
                let cond_str = mk_c_cond(cond, collected_errors, collected_vars, 0);
                write!(&mut code, "while({}) {{\n", cond_str).unwrap();
                code.push_str(&make_c(body, false, collected_vars, collected_errors, math_values));
                code.push_str("}\n");
            }
            AST::IF(body, cond) => {
                let cond_str = mk_c_cond(cond, collected_errors, collected_vars, 0);
                write!(&mut code, "if({}) {{\n", cond_str).unwrap();
                code.push_str(&make_c(body, false, collected_vars, collected_errors, math_values));
                code.push_str("}\n");
            }
            AST::VarAssign(var) => {
                use Variables::*;
                match var {
                    MATH(n, v) => {
                        let computed_value = eval_math_expr(v, math_values);
                        let new_type = if let Some(result) = computed_value {
                            if result.fract() == 0.0 {
                                if result >= i32::MIN as f64 && result <= i32::MAX as f64 {
                                    "i32"
                                } else {
                                    "i64"
                                }
                            } else {
                                let f32_val = result as f32;
                                if (f32_val as f64) != result {
                                    "f64"
                                } else {
                                    "f32"
                                }
                            }
                        } else {
                            if v.contains('.') {
                                "f32"
                            } else {
                                collected_vars.iter()
                                    .find(|(name, _)| name == n)
                                    .map(|(_, typ)| *typ)
                                    .unwrap_or("i32")
                            }
                        };
                        if let Some(result) = computed_value {
                            math_values.insert(n.clone(), result);
                        }
                        if new_type == "i32" || new_type == "i64" {
                            if let Some(result) = computed_value {
                                write!(&mut code, "{} = {};\n", n, result as i64).unwrap();
                            } else {
                                let v_clean = if v.ends_with(".0") { &v[..v.len()-2] } else { v };
                                write!(&mut code, "{} = {};\n", n, v_clean).unwrap();
                            }
                        } else {
                            write!(&mut code, "{} = {};\n", n, v).unwrap();
                        }
                    }
                    Char(n, v) => write!(&mut code, "{} = '{}';\n", n, v).unwrap(),
                    I8(n, v) => write!(&mut code, "{} = {};\n", n, v).unwrap(),
                    I16(n, v) => write!(&mut code, "{} = {};\n", n, v).unwrap(),
                    I32(n, v) => write!(&mut code, "{} = {};\n", n, v).unwrap(),
                    I64(n, v) => write!(&mut code, "{} = {};\n", n, v).unwrap(),
                    F32(n, v) => write!(&mut code, "{} = {};\n", n, v).unwrap(),
                    F64(n, v) => write!(&mut code, "{} = {};\n", n, v).unwrap(),
                    Str(n, v) => write!(&mut code, "{} = nstr_new(\"{}\");\n", n, v).unwrap(),
                    REF(n, v) => write!(&mut code, "{} = {};\n", n, v).unwrap(),
                }
            }
        }
    }
    if gen_main_function {
        code.push_str("return 0;\n}");
    }
    code
}

#[inline]
fn resolve_ref<'a>(ast: &'a [AST], mut current: &'a str) -> (&'static str, &'a str) {
    while let Some(next) = ast.iter().find_map(|node| {
        if let AST::Var(Variables::REF(name, next_val)) = node {
            if name == &current { Some(next_val.as_str()) } else { None }
        } else { None }
    }) {
        current = next;
    }
    let typ = ast.iter().find_map(|node| {
        if let AST::Var(var) = node {
            match var {
                Variables::Char(name, _) if &name[..] == current => Some("char"),
                Variables::I8(name, _) if &name[..] == current => Some("i8"),
                Variables::I16(name, _) if &name[..] == current => Some("i16"),
                Variables::I32(name, _) if &name[..] == current => Some("i32"),
                Variables::I64(name, _) if &name[..] == current => Some("i64"),
                Variables::F32(name, _) if &name[..] == current => Some("f32"),
                Variables::F64(name, _) if &name[..] == current => Some("double"),
                Variables::MATH(name, _) if &name[..] == current => Some("f32"),
                Variables::Str(name, _) if &name[..] == current => Some("nstring"),
                _ => None,
            }
        } else {
            None
        }
    }).unwrap_or("auto");
    (typ, current)
}
