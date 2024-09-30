use crate::utils::types::{Args, Tokens, Vars};
use std::collections::HashSet;

pub fn to_c(tokens: &Vec<Tokens>) -> String {
    let imports = String::from("#include <stdio.h>\n\n");
    let mut main = String::new();
    let mut funs = String::new();

    // Set to track declared variables in order to avoid redeclarations
    let mut declared_vars: HashSet<String> = HashSet::new();
    let mut printed: HashSet<String> = HashSet::new(); // Set to track printed values

    for i in tokens {
        match i {
            Tokens::Func(fun) => {
                // Create a Vec to store argument names
                let arg_vars: Vec<String> = fun
                    .args
                    .iter()
                    .map(|arg| match arg {
                        Args::Str(name) => name.clone(),
                        Args::Int(name) => name.clone(),
                        Args::Float(name) => name.clone(),
                        _ => unreachable!(),
                    })
                    .collect();

                // Generate C function header
                let s = format!("void {}({}) {{\n", fun.name, make_args(&fun.args));
                funs.push_str(&s);
                process(
                    &mut funs,
                    &arg_vars,
                    true,
                    &fun.code,
                    &mut declared_vars,
                    &mut printed,
                );
                funs.push_str("}\n\n"); // Close the function definition
            }
            _ => {
                // Generate main code
                process(
                    &mut main,
                    &vec![],
                    false,
                    tokens,
                    &mut declared_vars,
                    &mut printed,
                ); // No args for main
            }
        }
    }

    // Combine all parts into final C code
    let mut c_code = imports;
    c_code.push_str(&funs);
    c_code.push_str("int main() {\n");
    c_code.push_str(&main);
    c_code.push_str("    return 0;\n}\n"); // Close main function

    c_code
}

fn process(
    func: &mut String,
    arg_vars: &Vec<String>,
    _is_fun: bool,
    tokens: &Vec<Tokens>,
    declared_vars: &mut HashSet<String>,
    printed: &mut HashSet<String>, // Track printed statements to avoid duplicates
) {
    for token in tokens {
        match token {
            Tokens::Print(v, _n) => {
                // Check if this print statement has already been printed
                if !printed.contains(v) {
                    func.push_str(&format!("    printf(\"{}\");\n", v));
                    printed.insert(v.clone()); // Mark this print statement as printed
                }
            }
            Tokens::FnCall(fc) => {
                func.push_str(&format!("    {}();\n", fc));
            }
            Tokens::Var(v, n, mutable) => {
                // Skip redeclaring function arguments
                if arg_vars.contains(n) {
                    continue;
                }

                // Check if the variable has already been declared
                if declared_vars.contains(n) {
                    continue; // Skip if already declared
                }

                // Add the variable to the declared set
                declared_vars.insert(n.clone());

                // Generate variable declaration based on type and mutability
                let var_declaration = if !(*mutable) {
                    match v {
                        Vars::STR(s) => format!("char *{} = \"{}\";\n", n, s),
                        Vars::INT(s) => format!("int {} = {};\n", n, s),
                        Vars::F(f) => format!("double {} = {};\n", n, f),
                        _ => String::new(),
                    }
                } else {
                    // Immutable variables should be declared as 'const'
                    match v {
                        Vars::STR(s) => format!("const char *{} = \"{}\";\n", n, s),
                        Vars::INT(s) => format!("const int {} = {};\n", n, s),
                        Vars::F(f) => format!("const double {} = {};\n", n, f),
                        _ => String::new(),
                    }
                };

                // Add variable declaration to the function body
                func.push_str(&var_declaration);
            }
            Tokens::Revar(n, v) => {
                func.push_str(format!("{} = {};\n", n, v).as_str());
            }
            _ => {}
        }
    }
}

fn make_args(args: &Vec<Args>) -> String {
    let mut farg = String::new();
    for (i, arg) in args.iter().enumerate() {
        match arg {
            Args::Str(name) => farg.push_str(&format!("char *{}", name)),
            Args::Int(name) => farg.push_str(&format!("int {}", name)),
            Args::Float(name) => farg.push_str(&format!("double {}", name)),
            Args::EMP(_) => {} // No args in the function
            Args::E => {}      // Skip
        }
        if i < args.len() - 1 {
            farg.push_str(", "); // Add comma for separation
        }
    }
    farg
}
