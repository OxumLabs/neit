use std::{
    env::args,
    fs::File,
    io::Read,
    path::Path,
    process::exit,
};
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

#[allow(dead_code)]
pub struct Config {
    command: &'static str,
    path: &'static str,
    static_flag: bool,
    out: &'static str,
    targets: Vec<&'static str>,
    cc: &'static str,
}

fn normalize_target(input: &str) -> &'static str {
    match input.trim().to_lowercase().as_str() {
        "windows-x86-64" | "winx8664" => "windows-x86-64",
        "linux-x86-64" | "linx8664" => "linux-x86-64",
        other => Box::leak(other.to_string().into_boxed_str()),
    }
}

fn parse_config() -> Config {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        print_help();
        exit(1);
    }
    let command: &'static str = Box::leak(args[1].clone().into_boxed_str());
    let path: &'static str = Box::leak(args[2].clone().into_boxed_str());
    let mut static_flag = false;
    let mut out: &'static str = "out";
    let default_target = if cfg!(target_os = "windows") {
        "windows-x86-64"
    } else {
        "linux-x86-64"
    };
    let mut targets: Vec<&'static str> = vec![default_target];
    let mut cc: &'static str = "";
    for arg in args.iter().skip(3) {
        let arg_static: &'static str = Box::leak(arg.clone().into_boxed_str());
        if arg_static == "--static" {
            static_flag = true;
        } else if arg_static.starts_with("--out=") {
            out = Box::leak(arg_static["--out=".len()..].to_string().into_boxed_str());
        } else if arg_static.starts_with("--target=") {
            let value = &arg_static["--target=".len()..];
            let normalized: Vec<&'static str> = value
                .split(',')
                .map(|s| normalize_target(s))
                .collect();
            targets = normalized;
        } else if arg_static.starts_with("--cc=") {
            cc = Box::leak(arg_static["--cc=".len()..].to_string().into_boxed_str());
        } else {
            println!("┌[Warning] Unknown option '{}'", arg_static);
        }
    }
    Config { command, path, static_flag, out, targets, cc }
}

fn print_help() {
    println!("┌[*] Neit Programming Language - Help");
    println!("├─ Usage: neit <command> <file/folder> <options>");
    println!("├─ Commands:");
    println!("│   ├─ build   - Build a Neit source file/folder");
    println!("│   └─ help    - Display this help message");
    println!("├─ Options:");
    println!("│   ├─ --static                - Build with static linking");
    println!("│   ├─ --out=<file_name>       - Specify output file name (default: out)");
    println!("│   ├─ --target=<target>       - Specify target platform(s) (default: host OS)");
    println!("│   │        Acceptable targets for Windows: \"windows-x86-64\" or \"winx8664\"");
    println!("│   │        Acceptable targets for Linux:   \"linux-x86-64\" or \"linx8664\"");
    println!("│   │        Multiple targets can be separated by commas.");
    println!("│   └─ --cc=<compiler>         - Specify compiler (zig, clang, gcc)");
    println!("└─ Example: neit build ./source.neit --out=program --target=linux-x86-64,winx8664 --cc=zig");
}


fn main() {
    let config = parse_config();
    if config.command == "help" {
        print_help();
        exit(0);
    }
    if config.command != "build" && config.command != "neit" {
        eprintln!("┌[!!] Error");
        eprintln!("├─ Unknown command '{}'", config.command);
        eprintln!("└─ [~] Try using `neit help` for available commands.");
        exit(1);
    }
    println!("┌[*] Neit Build System - Build");
    println!("├─ Starting compilation of '{}'", config.path);
    let proj = Path::new(config.path);
    if !proj.exists() {
        eprintln!("┌[Error]");
        eprintln!("├─ The file or directory '{}' does not exist.", config.path);
        eprintln!("└─ Please check the path and try again.");
        exit(1);
    }
    if proj.is_dir() {
        eprintln!("┌[Error]");
        eprintln!("├─ Directories are not supported.");
        eprintln!("└─ Please provide a file path.");
        exit(1);
    }
    let mut file = match File::open(config.path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("┌[Error]");
            eprintln!("├─ Unable to open file '{}': {}", config.path, e);
            eprintln!("└─ Check file permissions.");
            exit(1);
        }
    };
    let mut code = String::new();
    if let Err(e) = file.read_to_string(&mut code) {
        eprintln!("┌[Error]");
        eprintln!("├─ Failed to read file '{}': {}", config.path, e);
        eprintln!("└─ Please ensure the file is accessible.");
        exit(1);
    }
    println!("├─ File '{}' read successfully.", config.path);
    let mut tokens: Vec<Token> = Vec::new();
    tokens.run_lexical_analysis(&code);
    println!("├─ Tokenization complete ({} tokens generated).", tokens.len());
    let ast = parse(&tokens, &code);
    println!("├─ Parsing complete. AST generated.");
    let c_code = make_c(&ast, true);
    println!("└─ C code generated. Initiating build process...");
    match linux_b_64(&c_code, &config) {
        Ok(()) => println!("└─ Build succeeded for '{}'.", config.path),
        Err(e) => {
            eprintln!("┌[Error]");
            eprintln!("├─ Build failed for '{}'.", config.path);
            eprintln!("└─ Error: {}", e);
            exit(1);
        }
    }
}
