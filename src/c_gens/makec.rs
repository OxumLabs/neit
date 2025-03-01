use crate::parse_systems::{PrintTokTypes, AST};
use std::fmt::Write;

/// Generates C code from the AST. If `gen_main_function` is true, a main function wrapper is produced
pub fn make_c(ast: &[AST], gen_main_function: bool) -> String {
    let mut code = String::with_capacity(1024);
    if gen_main_function {
        code.push_str("#include \"nulibc.h\"\n#include <stdio.h>\n#include <stdlib.h>\nint main(){\n");
    }
    for node in ast {
        let AST::Print { descriptor: fd, text } = node;
        let mut to_print = String::new();
        for ptok in text {
            match ptok {
                PrintTokTypes::Newline => to_print.push_str("\\n"),
                PrintTokTypes::Space => to_print.push(' '),
                PrintTokTypes::Var(var) => to_print.push_str(var),
                PrintTokTypes::Word(word) => to_print.push_str(word),
            }
        }
        write!(&mut code, "nprintf({},\"{}\");\n", fd.display(), to_print).unwrap();
    }
    if gen_main_function {
        code.push_str("return 0;\n}");
    }
    code
}