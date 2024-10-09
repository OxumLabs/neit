use crate::utils::{
    tokens::print::p_to_c,
    types::{Args, Tokens, Vars},
};
use std::{collections::HashSet, process::exit};

use super::bc::cfmt;

pub fn to_c(tokens: &Vec<Tokens>) -> String {
    let imports = String::from("#include <stdio.h>\n#include <string.h>\n");
    let mut main = String::new();
    let mut funs = String::new();

    // Add function definitions
    funs.push_str(
        r#"int fdi(int a, int b) {
    if (b == 0) {
        return 0; // Error: Division by zero
    }
    int result = a / b;
    if ((a % b != 0) && ((a < 0) != (b < 0))) {
        result--;
    }
    return result;
}

double fdf(double a, double b) {
    if (b == 0.0) {
        return 0.0; // Error: Division by zero in float
    }
    double result = a / b;
    return (result > 0 && result != (int)result) ? (int)result : (result < 0 && result != (int)result) ? (int)result - 1 : result;
}

"#,
    );

    let mut declared_vars: HashSet<String> = HashSet::new();

    // Handle function definitions
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

    // Generate main function
    main.push_str("int main() {\n");

    let non_function_tokens: Vec<&Tokens> = tokens
        .iter()
        .filter(|token| !matches!(token, Tokens::Func(_)))
        .collect();

    //println!("non func tkns:\n{:?}", non_function_tokens);

    // Process non-function tokens in global scope
    process(
        &mut main,
        &[],
        false,
        &non_function_tokens.iter().cloned().cloned().collect(),
        &mut declared_vars,
    );

    main.push_str("    return 0;\n}\n"); // Close main function

    // Combine all parts into final C code
    let mut c_code = imports;
    c_code.push_str(&funs);
    c_code.push_str(&main);
    c_code = cfmt(&c_code);
    c_code
}

fn process(
    func: &mut String,
    arg_vars: &[String],
    iff: bool, // Add the iff parameter
    tokens: &Vec<Tokens>,
    declared_vars: &mut HashSet<String>,
) {
    //let mut nli = 0;
    for token in tokens {
        match token {
            Tokens::IFun(_name, _code) => {
                // Only process Tokens::IFun if iff is true
                if iff {
                    let mut gcc = String::new();
                    process(&mut gcc, arg_vars, false, _code, declared_vars);
                    func.push_str(&gcc);
                }
            }
            Tokens::Cond(conds) => {
                let mut condc = String::new();
                let mut else_block = String::new();
                let mut last_condition_found = false;

                let mut addc = String::new(); // To store the code for each condition
                for (i, s) in conds.iter().enumerate() {
                    addc.clear(); // Clear addc for each new condition

                    // Skip empty conditions
                    if s.trim().is_empty() {
                        continue;
                    }

                    // Split condition into the condition part and the code part
                    let pts: Vec<&str> = s.split(":").collect();
                    if pts.len() != 2 {
                        eprintln!("Error! The Condition '{}' is invalid", s);
                        continue;
                    }

                    let cond = pts[0].trim(); // The condition part
                    let code = pts[1].trim(); // The code part

                    // Check for the 'last' condition
                    if cond == "last" {
                        //println!("in last condition");
                        if last_condition_found {
                            eprintln!("Error! Multiple 'last' conditions found.");
                            exit(1); // Exit if multiple 'last' conditions are found
                        }
                        last_condition_found = true;

                        // Only process tokens related to this 'last' condition
                        for t in tokens {
                            //println!("las cond token match : {:?}", t);
                            if let Tokens::IFun(n, c) = t {
                                if n == code {
                                    //println!("\ntoken matched last cond : {:?}", t);
                                    // Clear addc before processing to avoid carrying code from other conditions
                                    addc.clear();
                                    process(&mut addc, arg_vars, true, c, declared_vars);
                                    // Store the code for the 'last' condition in the else block
                                    else_block.push_str(&format!("    {}\n", addc));
                                }
                            }
                        }
                        continue; // Skip the rest of the processing for 'last'
                    }

                    // For the first condition, use 'if', otherwise 'else if'
                    if i == 0 {
                        condc.push_str(&format!("if ({}) {{\n", cond));
                    } else {
                        condc.push_str(&format!("else if ({}) {{\n", cond));
                    }

                    // Process the tokens for the current condition
                    for t in tokens {
                        if let Tokens::IFun(n, c) = t {
                            if n == code {
                                addc.clear(); // Clear addc before processing the current condition
                                process(&mut addc, arg_vars, true, c, declared_vars);
                                condc.push_str(&addc);
                            }
                        }
                    }

                    condc.push_str("}\n"); // Close the condition block
                }

                // If a 'last' condition was found, append the else block
                if last_condition_found {
                    condc.push_str("else {\n");
                    condc.push_str(&else_block);
                    condc.push_str("}\n");
                }

                // Finalize the generated code
                func.push_str(&condc);
            }

            Tokens::Print(v, _n) => {
                let pc = p_to_c(v, tokens);
                func.push_str(format!("    printf({});\n", pc).as_str());
            }
            Tokens::In(vnm) => {
                func.push_str(&format!("fgets({}, sizeof({}) - 1, stdin);\n", vnm, vnm));
                func.push_str(&format!(
                    "for (int i = 0; {}[i] != '\\0'; i++) {{\nif ({}[i] == '\\n') {}[i] = '\\0';\n}}\n",
                    vnm, vnm, vnm
                ));
            }

            Tokens::FnCall(fc, args) => {
                func.push_str(&format!("    {}({});\n", fc, args.join(",")));
            }
            Tokens::Var(v, n, mutable) => {
                if arg_vars.contains(n) || declared_vars.contains(n) {
                    continue;
                }

                declared_vars.insert(n.clone());

                let var_declaration = if *mutable {
                    match v {
                        Vars::STR(s) => format!("char {}[{}] = \"{}\";\n", n, n.len() + 333, s),
                        Vars::INT(s) => format!("int {} = {};\n", n, s),
                        Vars::F(f) => format!("double {} = {};\n", n, f),
                        _ => unreachable!("✘ Error: Unsupported variable type."),
                    }
                } else {
                    match v {
                        Vars::STR(s) => format!("const char *{} = \"{}\";\n", n, s),
                        Vars::INT(s) => format!("const int {} = {};\n", n, s),
                        Vars::F(f) => format!("const double {} = {};\n", n, f),
                        _ => unreachable!("✘ Error: Unsupported variable type."),
                    }
                };

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
            Args::EMP(_) => {}
            Args::E => {}
        }
        if i < args.len() - 1 {
            farg.push_str(", ");
        }
    }
    farg
}
