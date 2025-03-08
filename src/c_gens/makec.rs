use crate::{
    helpers::c_condmk::mk_c_cond,
    parse_systems::{PrintTokTypes, Variables, AST},
    err_system::err_types::ErrTypes,
};
use std::fmt::Write;
use std::collections::HashMap;

#[allow(non_snake_case)]
pub fn make_c(
    ast: &[AST],
    gen_main_function: bool,
    collected_vars: &Vec<(String, &'static str)>, // Changed to Vec for mk_c_cond compatibility
    collected_errors: &mut Vec<ErrTypes>,
) -> String {
    let mut code = String::with_capacity(4096);
    
    let var_types: HashMap<_, _> = collected_vars
        .iter()
        .map(|(name, typ)| (name.as_str(), *typ))
        .collect();

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
                    write!(&mut code, "nprintf({},\"{}\",{});\n", fd.display(), fmt, args.join(","))
                } else {
                    write!(&mut code, "nprintf({},\"{}\");\n", fd.display(), fmt)
                }.unwrap();
            }
            AST::Var(var) => {
                use Variables::*;
                match var {
                    MATH(n, v) => write!(&mut code, "f32 {} = {};\n", n, v),
                    Char(n, v) => write!(&mut code, "char {} = '{}';\n", n, v),
                    I8(n, v) => write!(&mut code, "i8 {} = {};\n", n, v),
                    I16(n, v) => write!(&mut code, "i16 {} = {};\n", n, v),
                    I32(n, v) => write!(&mut code, "i32 {} = {};\n", n, v),
                    I64(n, v) => write!(&mut code, "i64 {} = {};\n", n, v),
                    F32(n, v) => write!(&mut code, "f32 {} = {};\n", n, v),
                    F64(n, v) => write!(&mut code, "double {} = {};\n", n, v),
                    Str(n, v) => write!(&mut code, "nstring {} = nstr_new(\"{}\");\n", n, v),
                    REF(n, v) => {
                        let (typ, actual) = resolve_ref(ast, v);
                        write!(&mut code, "{} {} = {};\n", typ, n, actual)
                    }
                }.unwrap();
            }
            AST::While(body, cond) => {
                let cond_str = mk_c_cond(cond, collected_errors, collected_vars, 0);
                write!(&mut code, "while({}) {{\n", cond_str).unwrap();
                code.push_str(&make_c(body, false, collected_vars, collected_errors));
                code.push_str("}\n");
            }
            AST::IF(body, cond) => {
                let cond_str = mk_c_cond(cond, collected_errors, collected_vars, 0);
                write!(&mut code, "if({}) {{\n", cond_str).unwrap();
                code.push_str(&make_c(body, false, collected_vars, collected_errors));
                code.push_str("}\n");
            }
            AST::VarAssign(var) => {
                use Variables::*;
                match var {
                    MATH(n, v) => write!(&mut code, "{} = {};\n", n, v),
                    Char(n, v) => write!(&mut code, "{} = '{}';\n", n, v),
                    I8(n, v) => write!(&mut code, "{} = {};\n", n, v),
                    I16(n, v) => write!(&mut code, "{} = {};\n", n, v),
                    I32(n, v) => write!(&mut code, "{} = {};\n", n, v),
                    I64(n, v) => write!(&mut code, "{} = {};\n", n, v),
                    F32(n, v) => write!(&mut code, "{} = {};\n", n, v),
                    F64(n, v) => write!(&mut code, "{} = {};\n", n, v),
                    Str(n, v) => write!(&mut code, "{} = nstr_new(\"{}\");\n", n, v),
                    REF(n, v) => write!(&mut code, "{} = {};\n", n, v),
                }.unwrap();
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