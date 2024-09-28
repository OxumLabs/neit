use crate::utils::types::{Args, Tokens, Vars};

pub fn to_c(tokens: &Vec<Tokens>) -> String {
    let imports = String::from("#include <stdio.h>\n\n");
    let mut main = String::new();
    let mut funs = String::new();

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
                process(&mut funs, &arg_vars, true, &fun.code);
                funs.push_str("}\n\n"); // Close the function definition
            }
            _ => {
                // Generate main code
                process(&mut main, &vec![], false, tokens); // No args for main
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

fn process(func: &mut String, arg_vars: &Vec<String>, _is_fun: bool, tokens: &Vec<Tokens>) {
    for token in tokens {
        match token {
            Tokens::Print(v, _n) => {
                func.push_str(&format!("    printf(\"%s: %%s\\n\", {});\n", v));
            }
            Tokens::FnCall(fc) => {
                func.push_str(&format!("    {}();\n", fc));
            }
            Tokens::Var(v, n, mutable) => {
                // If it's a function, ensure we have proper argument handling
                if let Some(arg_index) = arg_vars.iter().position(|name| name == n) {
                    func.push_str(&format!("    {} = arg_vars[{}];\n", n, arg_index));
                } else {
                    // Generate variable declaration based on type
                    let var_declaration = match v {
                        Vars::STR(s) => format!("char *{} = \"{}\";\n", n, s),
                        Vars::INT(s) => format!("int {} = {};\n", n, s),
                        Vars::F(f) => format!("double {} = {};\n", n, f),
                        _ => String::new(), // Handle other types as needed
                    };

                    // Add variable declaration
                    func.push_str(&var_declaration);

                    // If mutable, we might want to include some initialization logic
                    if *mutable {
                        func.push_str(&format!("    {} = 0; // Initial value\n", n));
                    }
                }
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
