use std::{env::args, fs::File, io::Read, path::Path, process::exit};

use build_system::linux_b::linux_b_64;
use c_gens::makec::make_c;
use parse_systems::parse;
use tok_system::{lexer::LexicalAnalysis, tokens::Token};

pub mod build_system;
pub mod c_gens;
pub mod err_system;
pub mod nulibc;
pub mod parse_systems;
pub mod helpers;
pub mod tok_system;

fn print_help() {
    println!("┌[*] Neit Programming Language - Help");
    println!("├─ Usage: neit <command> <file_path>");
    println!("├─ Commands:");
    println!("│   ├─ build   - Build a Neit source file");
    println!("│   └─ help    - Display this help message");
    println!("└─ Example: neit build ./source.neit");
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        print_help();
        exit(1);
    }
    let cmd = &args[1];
    if cmd == "help" {
        print_help();
        exit(0);
    }
    if args.len() < 3 {
        print_help();
        exit(1);
    }
    let path = &args[2];
    if cmd == "neit" || cmd == "build" {
        println!("┌[*] Neit Build System - Build");
        println!("├─ Starting compilation of '{}'", path);
        let proj = Path::new(path);
        if !proj.exists() {
            eprintln!("┌[Error]");
            eprintln!("├─ The file or directory '{}' does not exist.", path);
            eprintln!("└─ Please check the path and try again.");
            exit(1);
        }
        if proj.is_dir() {
            eprintln!("┌[Error]");
            eprintln!("├─ Directories are not supported.");
            eprintln!("└─ Please provide a file path.");
            exit(1);
        }
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("┌[Error]");
                eprintln!("├─ Unable to open file '{}': {}", path, e);
                eprintln!("└─ Check file permissions.");
                exit(1);
            }
        };

        let mut code = String::new();
        if let Err(e) = file.read_to_string(&mut code) {
            eprintln!("┌[Error]");
            eprintln!("├─ Failed to read file '{}': {}", path, e);
            eprintln!("└─ Please ensure the file is accessible.");
            exit(1);
        }
        println!("├─ File '{}' read successfully.", path);

        let mut tokens: Vec<Token> = Vec::new();
        tokens.run_lexical_analysis(&code);
        println!("├─ Tokenization complete ({} tokens generated).", tokens.len());

        let ast = parse(&tokens, &code);
        println!("├─ Parsing complete. AST generated.");

        let c_code = make_c(&ast, true);
        println!("├─ C code generated. Initiating build process by gcc...");

        match linux_b_64(&c_code) {
            Ok(()) => println!("└─ Build succeeded for '{}'.", path),
            Err(e) => {
                eprintln!("┌[Error]");
                eprintln!("├─ Build failed for '{}'.", path);
                eprintln!("└─ Error: {}", e);
                exit(1);
            }
        }
    } else {
        eprintln!("┌[!!] Error");
        eprintln!("├─ Unknown command '{}'", cmd);
        eprintln!("└─ [~] Try using `neit help` for available commands.");
        exit(1);
    }
}
