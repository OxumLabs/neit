use crate::{
    helpers::c_condmk::mk_c_cond,
    parse_systems::{PrintTokTypes, Variables, AST, COLLECTED_VARS},
};
use std::fmt::Write;

pub fn make_c(ast: &[AST], gen_main_function: bool) -> String {
    let mut code = String::with_capacity(1024);
    if gen_main_function {
        code.push_str("#include \"nulibc.h\"\n#include <stdio.h>\n#include <stdlib.h>\nint main(){\n");
    }
    for node in ast {
        match node {
            AST::Print { descriptor: fd, text } => {
                let mut format_string = String::with_capacity(text.len() * 4);
                let mut args = Vec::new();
                let collected = COLLECTED_VARS.lock().unwrap();
                for ptok in text {
                    match ptok {
                        PrintTokTypes::Newline => format_string.push_str("\\n"),
                        PrintTokTypes::Space => format_string.push(' '),
                        PrintTokTypes::Word(word) => format_string.push_str(word),
                        PrintTokTypes::Var(var) => {
                            if let Some((_, var_type)) = collected.iter().find(|(name, _)| name == var) {
                                match *var_type {
                                    "ch" => format_string.push_str("%c"),
                                    "i8" | "i16" | "i32" | "i64" => format_string.push_str("%d"),
                                    "f32" => format_string.push_str("%f"),
                                    "f64" => format_string.push_str("%lf"),
                                    "str" => format_string.push_str("%s"),
                                    _ => {}
                                }
                            }
                            args.push(var.clone());
                        }
                    }
                }
                if !args.is_empty() {
                    write!(&mut code, "nprintf({},\"{}\",{});\n", fd.display(), format_string, args.join(",")).unwrap();
                } else {
                    write!(&mut code, "nprintf({},\"{}\");\n", fd.display(), format_string).unwrap();
                }
            }
            AST::Var(var) => {
                match var {
                    Variables::MATH(name, value) => {
                        write!(&mut code, "f32 {} = {};\n", name, value).unwrap();
                    }
                    Variables::Char(name, value) => {
                        write!(&mut code, "char {} = '{}';\n", name, value).unwrap();
                    }
                    Variables::I8(name, value) => {
                        write!(&mut code, "i8 {} = {};\n", name, value).unwrap();
                    }
                    Variables::I16(name, value) => {
                        write!(&mut code, "i16 {} = {};\n", name, value).unwrap();
                    }
                    Variables::I32(name, value) => {
                        write!(&mut code, "i32 {} = {};\n", name, value).unwrap();
                    }
                    Variables::I64(name, value) => {
                        write!(&mut code, "i64 {} = {};\n", name, value).unwrap();
                    }
                    Variables::F32(name, value) => {
                        write!(&mut code, "f32 {} = {}f;\n", name, value).unwrap();
                    }
                    Variables::F64(name, value) => {
                        write!(&mut code, "double {} = {};\n", name, value).unwrap();
                    }
                    Variables::Str(name, value) => {
                        write!(&mut code, "nstring {} = nstr_new(\"{}\");\n", name, value).unwrap();
                    }
                    Variables::REF(name, value) => {
                        let mut actual_value = value;
                        let mut actual_type = "auto";
                        loop {
                            if let Some(next) = ast.iter().find_map(|node| {
                                if let AST::Var(Variables::REF(var_name, next_value)) = node {
                                    if var_name == actual_value {
                                        Some(next_value)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }) {
                                actual_value = next;
                            } else {
                                break;
                            }
                        }
                        if let Some(var) = ast.iter().find(|node| {
                            if let AST::Var(variable) = node {
                                match variable {
                                    Variables::Char(var_name, _) => var_name == actual_value,
                                    Variables::I8(var_name, _) => var_name == actual_value,
                                    Variables::I16(var_name, _) => var_name == actual_value,
                                    Variables::I32(var_name, _) => var_name == actual_value,
                                    Variables::I64(var_name, _) => var_name == actual_value,
                                    Variables::F32(var_name, _) => var_name == actual_value,
                                    Variables::F64(var_name, _) => var_name == actual_value,
                                    Variables::REF(_, _) => false,
                                    Variables::MATH(var_name, _) => var_name == actual_value,
                                    Variables::Str(var_name, _) => var_name == actual_value,
                                }
                            } else {
                                false
                            }
                        }) {
                            actual_type = match var {
                                AST::Var(Variables::Char(_, _)) => "char",
                                AST::Var(Variables::I8(_, _)) => "i8",
                                AST::Var(Variables::I16(_, _)) => "i16",
                                AST::Var(Variables::I32(_, _)) => "i32",
                                AST::Var(Variables::I64(_, _)) => "i64",
                                AST::Var(Variables::F32(_, _)) => "f32",
                                AST::Var(Variables::F64(_, _)) => "double",
                                AST::Var(Variables::MATH(_, _)) => "f64",
                                AST::Var(Variables::Str(_, _)) => "str",
                                _ => "auto",
                            };
                        }
                        write!(&mut code, "{} {} = {};\n", actual_type, name, actual_value).unwrap();
                    }
                }
            }
            AST::While(body, cond) => {
                let cond = mk_c_cond(cond);
                write!(&mut code, "while({}) {{\n", cond).unwrap();
                let cond_code = make_c(&body, false);
                code.push_str(&cond_code);
                code.push_str("}\n");
            }
            AST::IF(body, cond) => {
                let cond = mk_c_cond(cond);
                write!(&mut code, "if({}) {{\n", cond).unwrap();
                let cond_code = make_c(&body, false);
                code.push_str(&cond_code);
                code.push_str("}\n");
            }
        }
    }
    if gen_main_function {
        code.push_str("return 0;\n}");
    }
    code
}
