use crate::utils::types::{Args, Tokens};

pub fn genasm(tokens: Vec<Tokens>) -> String {
    let mut asm = String::new();
    let mut data = String::new();
    let mut code = String::new();
    let mut funcs = String::new();

    // Data section (for initialized data)
    data.push_str("section .data\n");

    // Text section (for code)
    code.push_str("section .text\n");
    code.push_str("global _start\n"); // Declare _start as the entry point
    code.push_str("_start:\n");

    // Iterate over the tokens to generate assembly code
    for token in tokens {
        match token {
            Tokens::Func(func) => {
                // Handle function definitions
                let mut func_code = String::new();

                // Define the function label
                func_code.push_str(&format!("\n{}:\n", func.name));

                // Setup for handling function arguments
                for (i, arg) in func.args.iter().enumerate() {
                    match arg {
                        Args::Str(_) => {
                            // String arguments (pointer in rdi for the first argument)
                            if i == 0 {
                                func_code.push_str("    ; String argument (text) in rdi\n");
                            }
                        }
                        Args::Float(_) => {
                            // Handle floating-point arguments (need additional logic)
                            // TODO: Implement instructions for loading/manipulating xmm registers
                        }
                        Args::Int(_) => {
                            // Integer arguments (registers rdi, rsi, rdx, rcx, etc.)
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
                            // Initialize register with 64-bit mov
                        }
                        _ => {}
                    }
                }

                // Add function body (currently disabled)
                // TODO: Implement logic to translate func.code into assembly instructions

                // Add a return instruction
                func_code.push_str("    ret\n");

                // Add the function code to the funcs section
                funcs.push_str(&func_code);
            }
        }
    }

    // Add a simple exit syscall
    code.push_str("    mov rax, 60         ; syscall number for exit (sys_exit)\n");
    code.push_str("    mov rdi, 0          ; status code 0\n");
    code.push_str("    syscall             ; invoke syscall\n");

    // Combine all sections into the final assembly code
    asm.push_str(&data);
    asm.push_str(&code);
    asm.push_str(&funcs);

    // Return the assembled code as a result
    asm
}
