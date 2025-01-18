use build::build;
use colored::Colorize;
use enable_ansi_support::enable_ansi_support;
use help::help;
#[allow(unused)]
use lex::{lex, Tokens};
#[allow(unused)]
use nrunp::nrunp;
#[allow(unused)]
use p::parse;
use std::fs;
#[allow(unused)]
use std::{
    env::args,
    fs::File,
    process::{exit, Command},
};

pub mod build;
pub mod codegen;
mod err;
pub mod grm;
pub mod help;
mod lex;
pub mod nrunp;
pub mod nulibc;
mod p;
mod p2;
pub mod p3;
pub mod pbc;
mod run;

fn main() {
    match enable_ansi_support() {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{}",
                "Unable to enable ANSI color support:~ ANSI codes may appear inline, please ignore!"
                    .bright_yellow()
            );
            eprintln!("{}{}", "Specific Error Message:~ ", e);
        }
    }

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        help();
        exit(0);
    }
    let cmd = args[1].as_str();
    match cmd {
        "h" | "help" => help(),
        "build" => build(&args),
        "run" => {
            if args.len() < 3 {
                eprintln!("Error: No source file provided.");
                help();
                exit(1);
            }

            let srcf = format!("./{}", &args[2]);
            let rargs = ["joyjoy", "build", &srcf, "-o=run", "-rc"];

            // Start building the program
            println!("{}", "***** Starting the build process *****".dimmed());
            build(&rargs.iter().map(|s| s.to_string()).collect::<Vec<String>>());

            // Run the program
            println!("\n{}", "***** Running the program *****\n".dimmed());
            let mut rcmd = Command::new("./run");

            match rcmd.status() {
                Ok(status) => {
                    if status.success() {
                        println!("{}", "Program ran successfully.".bright_green());
                    } else {
                        println!("{}", "Program failed to run.".bright_red());
                    }
                }
                Err(e) => {
                    eprintln!("Error running program: {}", e);
                    return; // Exit early if running the program fails
                }
            }

            // Wait for the program to finish and then delete the file
            println!("{}", "***** Removing the program file *****".dimmed());
            match fs::remove_file("run") {
                Ok(_) => {
                    println!("{}", "Program successfully removed.".bright_green());
                }
                Err(e) => {
                    eprintln!("Error removing program: {}", e);
                }
            }

            // Indicate the completion of the process
            println!("{}", "***** Process completed *****".dimmed());
        }

        _ => {
            eprintln!("Error: Unknown Command: {}\n", cmd);
            help();
            exit(1);
        }
    }
}
