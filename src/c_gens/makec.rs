use crate::parse_systems::{PrintTokTypes, Variables, AST, COLLECTED_VARS};
use std::fmt::Write;

/// Generates C code from the AST. If `gen_main_function` is true, a main function wrapper is produced.
pub fn make_c(ast: &[AST], gen_main_function: bool) -> String {
    let mut code = String::with_capacity(1024);
    if gen_main_function {
        code.push_str(
            "#include \"nulibc.h\"\n#include <stdio.h>\n#include <stdlib.h>\nint main(){\n",
        );
    }
    for node in ast {
        if let AST::Print { descriptor: fd, text } = node {
            let mut to_print = String::new();
            let mut args_to_print = Vec::new();
            // Lock COLLECTED_VARS once per print statement.
            let collected = COLLECTED_VARS.lock().unwrap();
            for ptok in text {
                match ptok {
                    PrintTokTypes::Newline => to_print.push_str("\\n"),
                    PrintTokTypes::Space => to_print.push(' '),
                    PrintTokTypes::Var(var) => {
                        args_to_print.push(format!("{}", var));
                        if let Some((_, var_type)) = collected.iter().find(|(name, _)| name == var) {
                            match *var_type {
                                "ch" => write!(to_print, "%c").unwrap(),
                                "i8" | "i16" | "i32" | "i64" => write!(to_print, "%d").unwrap(),
                                "f32" => write!(to_print, "%f").unwrap(),
                                "f64" => write!(to_print, "%lf").unwrap(),
                                _ => {}
                            }
                        }
                        // Append closing quote and comma for arguments.
                        to_print.push_str("\",");
                        to_print.push_str(&args_to_print.join(","));
                    },
                    PrintTokTypes::Word(word) => to_print.push_str(word),
                }
            }
            write!(&mut code, "nprintf({},\"{});\n", fd.display(), to_print).unwrap();
        } else if let AST::Var(var) = node {
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
                            _ => "auto",
                        };
                    }
                    write!(&mut code, "{} {} = {};\n", actual_type, name, actual_value).unwrap();
                }
            }
        }
    }
    if gen_main_function {
        code.push_str("return 0;\n}");
    }
    code
}
