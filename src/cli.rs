use std::{
    fs::{self, File},
    io::{self, stdin, stdout, Write},
    process::{exit, Command, Stdio},
};

use crate::{
    compilers::llvm::c::to_c,
    utils::{token::gentoken, types::Tokens},
};

pub fn cli() {
    // Intro message including help information
    println!("===============================");
    println!("  Welcome to the Neit CLI!  ");
    println!("===============================\n");
    println!("- Here, you can quickly test your Neit code and explore its features!");
    println!("- Use the command prompt to get started.");
    println!("- Type 'help' for a list of commands and their descriptions.");
    println!("- Enjoy coding with Neit!");
    println!("Type 'cls' to clear the screen or 'leave' to exit the CLI (Case Sensitive)");

    let mut str_tkns: Vec<Tokens> = Vec::new();
    let mut vars_clct: Vec<Tokens> = Vec::new();
    let mut store_mode = false; // New variable to track store mode

    loop {
        // Debugging prints to check the current state of tokens and variables
        println!("str_tkns b4 clear : {:?}", str_tkns);
        println!("vars_clct : {:?}", vars_clct);

        // Clear str_tkns if store_mode is off
        if !store_mode {
            str_tkns.clear();
            println!("str_tkns after clear : {:?}", str_tkns);
        }

        let mut cmd = String::new();
        print!("> ");
        stdout().flush().unwrap();

        match stdin().read_line(&mut cmd) {
            Ok(_) => {
                let trimmed_cmd = cmd.trim();

                match trimmed_cmd {
                    "cls" => clear_screen(),
                    "leave" | "ext" | "exit" => exit(0),
                    "help" => display_help(), // Call function to display help
                    "storemode" => {
                        store_mode = toggle_store_mode(); // Call function to toggle store mode
                    }
                    _ => {
                        // Process Neit code
                        let olv = vec![trimmed_cmd.to_string()];
                        match gentoken(olv, str_tkns.clone(), false) {
                            Ok(mut tkn) => {
                                // Store variables for later usage
                                for i in tkn.clone() {
                                    match i {
                                        Tokens::Var(_, _, _) | Tokens::Revar(_, _) => {
                                            vars_clct.push(i);
                                        }
                                        _ => {}
                                    }
                                }

                                // Only extend if tokens are not empty
                                if !vars_clct.is_empty() {
                                    tkn.extend_from_slice(&vars_clct);
                                }

                                println!("tkns extended : {:?}", tkn);
                                let c_code = to_c(&tkn);
                                compile_and_run(c_code);
                            }
                            Err(e) => eprintln!("{}", e),
                        }
                    }
                }
            }
            Err(e) => {
                handle_input_error(e);
            }
        }
    }
}

// Function to toggle store mode
fn toggle_store_mode() -> bool {
    let mut input = String::new();
    println!("Turn store mode on or off? (type 'on' or 'off')");
    print!("storemode > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match input {
        "on" => {
            println!("Store mode is now ON. Tokens will be stored.");
            true // Return true to indicate store mode is enabled
        }
        "off" => {
            println!("Store mode is now OFF. Tokens will be cleared.");
            false // Return false to indicate store mode is disabled
        }
        _ => {
            println!("Invalid input. Please type 'on' or 'off'.");
            false // Default to off if input is invalid
        }
    }
}

// Function to display help information
fn display_help() {
    println!("===============================");
    println!("          Neit CLI Help        ");
    println!("===============================\n");
    println!("Available commands:");
    println!("  cls       - Clear the screen.");
    println!("  leave     - Exit the CLI.");
    println!("  help      - Display this help message.");
    println!("  storemode - Turn token storage on or off.");
    println!("\nType your Neit code directly to compile and run it.");
    println!("Tokens will be stored based on the current store mode setting.");
}

fn compile_and_run(c_code: String) {
    match File::create("_.c") {
        Ok(mut tcf) => {
            if tcf.write_all(c_code.as_bytes()).is_err() {
                eprintln!("Error: Failed to write C code to the file. Do you have permissions?");
                return;
            }

            // Try to compile with clang first
            let clang_exists = run_command("clang", &["-o", "_.out", "_.c"]);

            if !clang_exists {
                // Fall back to gcc if clang isn't found
                if !run_command("gcc", &["-o", "_.out", "_.c"]) {
                    eprintln!("Error: Both clang and gcc failed to compile the file.");
                    return;
                }
            }

            // Run the compiled executable
            if let Ok(output) = Command::new("./_.out")
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
            {
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            } else {
                eprintln!("Error: Failed to execute the compiled program.");
            }

            // Cleanup generated files
            cleanup();
        }
        Err(_) => {
            eprintln!("Error: Failed to create the C source file. Do you have permissions?");
        }
    }
}

fn cleanup() {
    fs::remove_file("_.c").ok(); // Ignore errors if file removal fails
    fs::remove_file("_.out").ok();
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg("cls").status().ok();
    } else {
        Command::new("clear").status().ok();
    }
}

fn run_command(command: &str, args: &[&str]) -> bool {
    match Command::new(command).args(args).status() {
        Ok(status) => status.success(),
        Err(_) => {
            eprintln!("Error: {} is not installed or not found in PATH.", command);
            false
        }
    }
}

fn handle_input_error(e: std::io::Error) {
    eprintln!("Error reading input: {}", e);
    eprintln!(" - This could be due to an issue with the standard input stream.");
    eprintln!(
        " - Please check if your terminal is functioning properly or if the input was interrupted."
    );
    exit(1);
}
