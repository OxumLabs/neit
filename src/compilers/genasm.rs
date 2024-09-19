use crate::utils::types::{Args, Tokens};

pub fn genasm(tokens: Vec<Tokens>) -> String {
    let mut asm = String::new();
    let mut data = String::new();
    let mut code = String::new();

    let mut functions: Vec<(String, String, Vec<Tokens>)> = Vec::new(); // (func_name, asm_code, func_code_tokens)
    let mut fncalls: Vec<String> = Vec::new(); // Function calls

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
                functions.push((func.name.clone(), func_code, func.code.clone()));
            }
            Tokens::FnCall(ref nm) => {
                // Collect function calls
                fncalls.push(nm.clone());
            }
            _ => parse(
                &mut code.clone(),
                &mut code,
                false,
                token,
                &tokens,
                &mut data,
            ), // Handle other tokens
        }
    }

    // Second pass: Validate function calls and definitions
    let mut final_functions: Vec<(String, String)> = Vec::new(); // Final valid functions (name, asm_code)

    for (func_name, mut func_code, func_tokens) in functions {
        if fncalls.contains(&func_name) {
            // Check if function has code and is called
            if !func_tokens.is_empty() {
                // If function has code, parse the function body and add to final functions
                for token in func_tokens {
                    parse(&mut func_code, &mut code, true, token, &tokens, &mut data);
                }
                func_code.push_str("    ret\n"); // Add return instruction
                final_functions.push((func_name, func_code)); // Add to final list
            } else {
                // If function is empty, remove the function call and do not add the function
                fncalls.retain(|call| call != &func_name);
            }
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
) {
    match token {
        Tokens::Var(var, name) => {
            let vasm = var.to_asm(name);
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

            // Add the string to the data section
            data.push_str(&format!("    {} db '{}'\n", name, t));

            // Generate assembly code to print the string
            let print_code = format!(
                "    mov rax, 1\n    mov rdi, 1\n    mov rsi, {}\n    mov rdx, {}\n    syscall\n",
                name,
                t.len()
            );

            // Add to appropriate section (function body or main code)
            if inf {
                fnbody.push_str(&print_code);
            } else {
                code.push_str(&print_code);
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
