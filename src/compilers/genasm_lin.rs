use crate::utils::types::{Args, Tokens};
use std::collections::HashSet;

pub fn genasm_lin(tokens: &Vec<Tokens>) -> String {
    let mut asm = String::new();
    let mut data = String::new();
    let mut code = String::new();

    let mut functions: Vec<(String, String, Vec<Tokens>, bool)> = Vec::new();
    let mut counter = 0;
    let mut added_data: HashSet<String> = HashSet::new();

    data.push_str("section .data\n");
    code.push_str("section .text\n");
    code.push_str("global _start\n");
    code.push_str("_start:\n");

    for token in tokens.clone() {
        match token {
            Tokens::Func(ref func) => {
                let mut func_code = String::new();
                let has_vars = !func.local_vars.is_empty();

                if !func.is_global {
                    func_code.push_str(&format!("\n{}:\n", func.name));
                } else {
                    func_code.push_str(&format!("\nglobal {}\n{}:\n", func.name, func.name));
                }

                for (i, arg) in func.args.iter().enumerate() {
                    match arg {
                        Args::Str(_) => {
                            if i == 0 {
                                func_code.push_str("    ; String argument (text) in rdi\n");
                            }
                        }
                        Args::Float(_) => {
                            if i == 0 {
                                func_code.push_str("    ; Floating-point argument in xmm0\n");
                            }
                        }
                        Args::Int(_) => {
                            let reg = match i {
                                0 => "rdi",
                                1 => "rsi",
                                2 => "rdx",
                                3 => "rcx",
                                4 => "r8",
                                5 => "r9",
                                _ => "rax",
                            };
                            func_code.push_str(&format!("    mov {}, 0\n", reg));
                        }
                        _ => {}
                    }
                }

                functions.push((func.name.clone(), func_code, func.code.clone(), has_vars));
            }
            Tokens::FnCall(ref nm, _args) => {
                let mut call_code = String::new();
                let args = get_function_args(nm, tokens);

                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        Args::Str(_) => {}
                        Args::Float(_) => {
                            if i == 0 {
                                call_code.push_str("    movaps xmm0, [arg_float]\n");
                            }
                        }
                        Args::Int(_) => {
                            let reg = match i {
                                0 => "rdi",
                                1 => "rsi",
                                2 => "rdx",
                                3 => "rcx",
                                4 => "r8",
                                5 => "r9",
                                _ => "rax",
                            };
                            call_code.push_str(&format!("    mov {}, 0\n", reg));
                        }
                        _ => {}
                    }
                }

                call_code.push_str(&format!("    call {}\n", nm));
                code.push_str(&call_code);
            }
            _ => {
                parse(
                    &mut code.clone(),
                    &mut code,
                    false,
                    token,
                    tokens,
                    &mut data,
                    counter,
                    &mut added_data,
                );
                counter += 5;
            }
        }
    }

    let mut final_functions: Vec<(String, String)> = Vec::new();

    for (func_name, mut func_code, func_tokens, has_vars) in functions {
        if !func_tokens.is_empty() || has_vars {
            for token in func_tokens {
                parse(
                    &mut func_code,
                    &mut code,
                    true,
                    token,
                    tokens,
                    &mut data,
                    counter,
                    &mut added_data,
                );
            }
            func_code.push_str("    ret\n");
            final_functions.push((func_name, func_code));
        }
    }

    code.push_str("    mov rax, 60         ; syscall number for exit (sys_exit)\n");
    code.push_str("    mov rdi, 0          ; status code 0\n");
    code.push_str("    syscall             ; invoke syscall\n");

    asm.push_str(&data);
    asm.push_str(&code);

    for (_, func_code) in final_functions {
        asm.push_str(&func_code);
    }

    asm
}

fn parse(
    fnbody: &mut String,
    code: &mut String,
    inf: bool,
    token: Tokens,
    tokens: &Vec<Tokens>,
    data: &mut String,
    counter: i32,
    added_data: &mut HashSet<String>,
) {
    match token {
        Tokens::Var(var, name, _) => {
            let vasm = var.to_asm(name, counter);
            data.push_str(vasm.as_str());
        }
        Tokens::Print(txt, name) => {
            let processed_text = txt;
            // Prepare the text for .data section
            let data_key = format!("{}_{}", name, counter);
            if !added_data.contains(&data_key) {
                // Convert '\n' to 0xA (newline in ASCII) in assembly
                //println!("processed text : {}", processed_text);
                let asm_string = processed_text.replace("\\n", "', 0xA, '");
                data.push_str(&format!("    {} db '{}', 0\n", data_key, asm_string));
                added_data.insert(data_key.clone());
            }

            // Prepare the code for printing the string
            let print_code = format!(
                "    mov rax, 1\n    mov rdi, 1\n    mov rsi, {}\n    mov rdx, {}\n    syscall\n",
                data_key,
                processed_text.len()
            );

            // Append to the function body or main code
            if inf {
                if !fnbody.contains(&print_code) {
                    fnbody.push_str(&print_code);
                }
            } else if !code.contains(&print_code) {
                code.push_str(&print_code);
            }
        }
        Tokens::FnCall(nm, _args) => {
            let mut call_code = String::new();
            let args = get_function_args(&nm, tokens);

            for (i, arg) in args.iter().enumerate() {
                match arg {
                    Args::Str(_) => {}
                    Args::Float(_) => {
                        if i == 0 {
                            call_code.push_str("    movaps xmm0, [arg_float]\n");
                        }
                    }
                    Args::Int(_) => {
                        let reg = match i {
                            0 => "rdi",
                            1 => "rsi",
                            2 => "rdx",
                            3 => "rcx",
                            4 => "r8",
                            5 => "r9",
                            _ => "rax",
                        };
                        call_code.push_str(&format!("    mov {}, 0\n", reg));
                    }
                    _ => {}
                }
            }

            call_code.push_str(&format!("    call {}\n", nm));

            if inf {
                fnbody.push_str(&call_code);
            } else {
                code.push_str(&call_code);
            }
        }
        _ => {}
    }
}

fn get_function_args(name: &str, tokens: &[Tokens]) -> Vec<Args> {
    for token in tokens {
        if let Tokens::Func(func) = token {
            if name == func.name {
                return func.args.clone();
            }
        }
    }
    eprintln!("Error: Function '{}' not found.", name);
    std::process::exit(1);
}
