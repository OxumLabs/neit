use crate::utils::types::{Args, Tokens};
use std::collections::HashSet;

pub fn genasm_lin(tokens: Vec<Tokens>) -> String {
    let mut asm = String::new();
    let mut data = String::new();
    let mut code = String::new();

    let mut functions: Vec<(String, String, Vec<Tokens>, bool)> = Vec::new(); // (func_name, asm_code, func_code_tokens, has_vars)
    let mut counter = 0;
    let mut added_data: HashSet<String> = HashSet::new(); // Track data that has already been added

    // Data section (for initialized data)
    data.push_str("section .data\n");

    // Text section (for code)
    code.push_str("section .text\n");
    code.push_str("global _start\n"); // Declare _start as the entry point
    code.push_str("_start:\n");

    // First pass: collect all function definitions and calls
    for token in tokens.clone() {
        match token {
            Tokens::Func(ref func) => {
                let mut func_code = String::new();
                let has_vars = !func.local_vars.is_empty(); // Check if function has local variables

                // Define the function
                if !func.is_global {
                    func_code.push_str(&format!("\n{}:\n", func.name));
                } else {
                    func_code.push_str(&format!("\nglobal {}\n{}:\n", func.name, func.name));
                }

                // Setup for handling function arguments
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
                                _ => "rax", // Default register for more than 6 arguments
                            };
                            func_code.push_str(&format!("    mov {}, 0\n", reg));
                        }
                        _ => {}
                    }
                }

                // Store function definitions and code separately
                functions.push((func.name.clone(), func_code, func.code.clone(), has_vars));
            }
            Tokens::FnCall(ref nm) => {
                // Process function calls outside of functions (i.e., in the main code)
                let mut call_code = String::new();
                let args = get_function_args(nm, &tokens);

                // Handle function arguments
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
                                _ => "rax", // Default register for more than 6 arguments
                            };
                            call_code.push_str(&format!("    mov {}, 0\n", reg));
                        }
                        _ => {}
                    }
                }

                // Generate call instruction
                call_code.push_str(&format!("    call {}\n", nm));

                // Add the function call directly to the main code (_start)
                code.push_str(&call_code);
            }
            _ => {
                // Handle other tokens (e.g., variables, print)
                parse(
                    &mut code.clone(),
                    &mut code,
                    false,
                    token,
                    &tokens,
                    &mut data,
                    counter,
                    &mut added_data, // Pass the data tracking set
                );
                counter += 5;
            }
        }
    }

    // Second pass: Generate function code
    let mut final_functions: Vec<(String, String)> = Vec::new(); // Final valid functions (name, asm_code)

    for (func_name, mut func_code, func_tokens, has_vars) in functions {
        if !func_tokens.is_empty() || has_vars {
            // Check if function has code or local variables
            for token in func_tokens {
                parse(
                    &mut func_code,
                    &mut code,
                    true,
                    token,
                    &tokens,
                    &mut data,
                    counter,
                    &mut added_data,
                );
            }
            func_code.push_str("    ret\n"); // Add return instruction
            final_functions.push((func_name, func_code)); // Add to final list
        }
    }

    // Add a simple exit syscall at the end of the main code
    code.push_str("    mov rax, 60         ; syscall number for exit (sys_exit)\n");
    code.push_str("    mov rdi, 0          ; status code 0\n");
    code.push_str("    syscall             ; invoke syscall\n");

    // Combine all sections into the final assembly code
    asm.push_str(&data);
    asm.push_str(&code);

    // Append valid functions' code to the asm
    for (_, func_code) in final_functions {
        asm.push_str(&func_code);
    }

    // Return the final assembled code
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
    added_data: &mut HashSet<String>, // Add this parameter to track added data
) {
    match token {
        Tokens::Var(var, name) => {
            let vasm = var.to_asm(name, counter);
            data.push_str(&vasm.as_str());
        }
        Tokens::Print(txt, name) => {
            let mut t = String::new();
            let mut eso = false;

            for ch in txt.chars() {
                if eso {
                    match ch {
                        'n' => t.push_str("',0xA,'"), // Newline escape sequence
                        '\\' => t.push('\\'),         // Escaped backslash
                        _ => t.push(ch),              // Other escape sequences
                    }
                    eso = false;
                } else if ch == '\\' {
                    eso = true; // Start of escape sequence
                } else {
                    t.push(ch); // Normal character
                }
            }

            // Remove trailing commas and extra single quotes
            if t.ends_with(",''") {
                t = t.trim_end_matches(",''").to_string();
            }

            // Create a unique key for the data section
            let data_key = format!("{}_{}", name, counter);

            // Only add the data if it hasn't been added already
            if !added_data.contains(&data_key) {
                data.push_str(&format!("    {} db '{}'\n", data_key, t));
                added_data.insert(data_key.clone()); // Mark this data as added
            }

            // Generate assembly code to print the string
            let print_code = format!(
                "    mov rax, 1\n    mov rdi, 1\n    mov rsi, {}\n    mov rdx, {}\n    syscall\n",
                data_key,
                t.len()
            );

            // Add to appropriate section (function body or main code)
            if inf {
                if !fnbody.contains(&print_code) {
                    // Check if print code is already present
                    fnbody.push_str(&print_code);
                }
            } else {
                if !code.contains(&print_code) {
                    // Check if print code is already present
                    code.push_str(&print_code);
                }
            }
        }
        Tokens::FnCall(nm) => {
            let mut call_code = String::new();
            let args = get_function_args(&nm, tokens);

            // Handle function arguments
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
                            _ => "rax", // Default register for more than 6 arguments
                        };
                        call_code.push_str(&format!("    mov {}, 0\n", reg));
                    }
                    _ => {}
                }
            }

            // Generate call instruction
            call_code.push_str(&format!("    call {}\n", nm));

            // Add call code to the appropriate section
            if inf {
                fnbody.push_str(&call_code);
            } else {
                code.push_str(&call_code);
            }
        }
        _ => {}
    }
}

// Function to get function arguments from tokens
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
