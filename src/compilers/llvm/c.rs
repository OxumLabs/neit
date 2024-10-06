use crate::utils::{
    tokens::print::p_to_c,
    types::{Args, Tokens, Vars},
};
use std::collections::HashSet;

pub fn to_c(tokens: &Vec<Tokens>) -> String {
    let imports = String::from("#include <stdio.h>\n#include <string.h>\n");
    let mut main = String::new();
    let mut funs = String::new();

    funs.push_str(
        r#"int fdi(int a, int b) {
    if (b == 0) {
        // ✘ Error: Division by zero
        // → Hint: Handle error (e.g., return 0 or some error code)
        return 0;
    }
    int result = a / b;
    // ℹ Info: Adjust result if a and b have different signs and there's a remainder
    if ((a % b != 0) && ((a < 0) != (b < 0))) {
        result--;
    }
    return result;
}

"#,
    );

    // Define floor division for floats
    funs.push_str(
        r#"double fdf(double a, double b) {
    
    if (b == 0.0) {
        // ✘ Error: Division by zero in float
        return 0.0;
    }
    
    double result = a / b;
    if (result > 0 && result != (int)result) {
        return (int)result; // ℹ Info: Truncates towards zero
    }
    if (result < 0 && result != (int)result) {
        return (int)result - 1;
    }
    
    return result;
}

"#,
    );

    // Set to track declared variables in order to avoid redeclarations
    let mut declared_vars: HashSet<String> = HashSet::new();

    // First, handle function definitions
    for i in tokens {
        if let Tokens::Func(fun) = i {
            let arg_vars: Vec<String> = fun
                .args
                .iter()
                .map(|arg| match arg {
                    Args::Str(name) => name.clone(),
                    Args::Int(name) => name.clone(),
                    Args::Float(name) => name.clone(),
                    _ => unreachable!(
                        "✘ Error: Unsupported argument type. ⚙ Location: to_c make_args"
                    ),
                })
                .collect();

            // Generate C function header
            let s = format!("void {}({}) {{\n", fun.name, make_args(&fun.args));
            funs.push_str(&s);

            // Process function code (function body)
            process(&mut funs, &arg_vars, true, &fun.code, &mut declared_vars);

            // Close the function definition
            funs.push_str("\n}\n\n");
        }
    }

    // Now handle the main function generation
    main.push_str("int main() {\n");

    // Filter out function tokens so they are not processed in main
    let non_function_tokens: Vec<&Tokens> = tokens
        .iter()
        .filter(|token| !matches!(token, Tokens::Func(_)))
        .collect();

    // Process the non-function tokens in the global scope
    process(
        &mut main,
        &[],
        false,
        &non_function_tokens.iter().cloned().cloned().collect(),
        &mut declared_vars,
    ); // No args for main
    main.push_str("    return 0;\n}\n"); // Close main function

    // Combine all parts into final C code
    let mut c_code = imports;
    c_code.push_str(&funs);
    c_code.push_str(&main);

    c_code
}

fn process(
    func: &mut String,
    arg_vars: &[String],
    _is_fun: bool,
    tokens: &Vec<Tokens>,
    declared_vars: &mut HashSet<String>,
) {
    for token in tokens {
        match token {
            Tokens::Print(v, _n) => {
                //println!("v : {} |", v);
                let pc = p_to_c(v, tokens);
                let pc = format!("    printf({});\n", pc); // Add a newline after printf
                func.push_str(&pc);
            }
            Tokens::In(vnm) => {
                func.push_str(&format!(
                    "//printf(\"\\n\");\nfgets({}, sizeof({}) - 1, stdin);\n",
                    vnm, vnm
                ));
                func.push_str(&format!(
                    "char *newline = strchr({}, '\\n');\nif (newline) *newline = '\\0';\n",
                    vnm
                ));
            }
            Tokens::FnCall(fc, args) => {
                func.push_str(&format!("    {}({});\n", fc, args.join(",")));
            }
            Tokens::Var(v, n, mutable) => {
                if arg_vars.contains(n) {
                    continue;
                }
                if declared_vars.contains(n) {
                    continue; // Skip if already declared
                }

                // Add the variable to the declared set
                declared_vars.insert(n.clone());

                // Generate variable declaration based on type and mutability
                let var_declaration = if *mutable {
                    match v {
                        Vars::STR(s) => format!("char {}[{}] = \"{}\";\n", n, n.len() + 3333, s),
                        Vars::INT(s) => format!("int {} = {};\n", n, s),
                        Vars::F(f) => format!("double {} = {};\n", n, f),
                        _ => unreachable!(
                            "✘ Error: Unsupported variable type. ⚙ Location: process mutable"
                        ),
                    }
                } else {
                    match v {
                        Vars::STR(s) => format!("const char *{} = \"{}\";\n", n, s),
                        Vars::INT(s) => format!("const int {} = {};\n", n, s),
                        Vars::F(f) => format!("const double {} = {};\n", n, f),
                        _ => unreachable!(
                            "✘ Error: Unsupported variable type. ⚙ Location: process immutable"
                        ),
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

fn make_args(args: &[Args]) -> String {
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
