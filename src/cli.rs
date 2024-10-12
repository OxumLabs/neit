use std::{
    io::{stdin, stdout, Write},
    process::{exit, Command},
};

use crate::{
    compilers::llvm::c::to_c,
    utils::{token::gentoken, types::Tokens},
};

pub fn cli() {
    println!("===============================");
    println!("  Welcome to the Neit CLI!  ");
    println!("===============================\n");
    println!("- Here, you can quickly test your Neit code and explore its features!");
    println!("- Use the command prompt to get started.");
    println!("- Enjoy coding with Neit!");
    println!("Type 'cls' to clear the screen or 'leave' to exit the CLI (Case Sensitive)");
    let mut str_tkns: Vec<Tokens> = Vec::new();
    loop {
        let mut cmd = String::new();
        print!("> ");
        stdout().flush().unwrap();
        match stdin().read_line(&mut cmd) {
            Ok(_) => match cmd.trim() {
                "cls" => {
                    clear_screen();
                }
                "leave" | "ext" | "exit" => {
                    exit(0);
                }
                _ => {
                    let olv = vec![cmd.trim().to_string()];
                    let tkn = gentoken(olv, str_tkns.clone(), false);
                    match tkn {
                        Ok(tkn) => {
                            for i in &tkn {
                                str_tkns.push(i.clone());
                            }
                            let c = to_c(&tkn);
                            println!("c code : {}", c);
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                eprintln!(" - This could be due to an issue with the standard input stream.");
                eprintln!(" - Please check if your terminal is functioning properly or if the input was interrupted.");
                eprintln!(" - Ensure that you are using a compatible terminal environment that supports input operations.");
                exit(1);
            }
        }
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        // For Windows
        match Command::new("cmd").arg("/C").arg("cls").status() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Unable to Clear screen : error - {}", e);
            }
        }
    } else {
        // For Unix-like systems (Linux, macOS)
        Command::new("clear")
            .status()
            .expect("Failed to clear the screen");
    }
}
