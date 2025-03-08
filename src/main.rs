use std::{
    env::args,
    fs::File,
    io::Read,
    path::Path,
    process::exit, time::Duration,
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

#[inline(always)]
fn normalize_target(input: &str) -> &'static str {
    match input.trim().to_lowercase().as_str() {
        "windows-x86-64" | "winx8664" => "windows-x86-64",
        "linux-x86-64" | "linx8664"   => "linux-x86-64",
        other => Box::leak(other.to_string().into_boxed_str()),
    }
}

#[inline(always)]
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
            targets = value.split(',').map(|s| normalize_target(s)).collect();
        } else if arg_static.starts_with("--cc=") {
            cc = Box::leak(arg_static["--cc=".len()..].to_string().into_boxed_str());
        } else {
            println!("┌[Warning] Unknown option '{}'", arg_static);
        }
    }
    Config { command, path, static_flag, out, targets, cc }
}

#[inline(always)]
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
#[inline(always)]
fn main_logic() {
    use std::time::Instant;
    
    let total_start = Instant::now();
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
    let mut file = File::open(config.path).unwrap_or_else(|e| {
        eprintln!("┌[Error]");
        eprintln!("├─ Unable to open file '{}': {}", config.path, e);
        eprintln!("└─ Check file permissions.");
        exit(1);
    });
    let mut code = String::new();
    if file.read_to_string(&mut code).is_err() {
        eprintln!("┌[Error]");
        eprintln!("├─ Failed to read file '{}'", config.path);
        eprintln!("└─ Please ensure the file is accessible.");
        exit(1);
    }
    println!("├─ File '{}' read successfully.", config.path);
    let mut tokens: Vec<Token> = Vec::new();
    tokens.run_lexical_analysis(&code);
    println!("├─ Tokenization complete ({} tokens generated).", tokens.len());
    let proj_path: &'static str = Box::leak(proj.display().to_string().into_boxed_str());
    
    let mut collected_vars = Vec::new();
    let mut collected_errors = Vec::new();
    
    let (ast, _, _) = parse(&tokens, &code, proj_path, false, &mut collected_vars, &mut collected_errors);
    println!("├─ Parsing complete. AST generated.");
    let c_code = make_c(&ast, true, &collected_vars, &mut collected_errors);
    println!("├─ C code generated");
    fn format_duration(duration: Duration) -> String {
        let nanos = duration.as_nanos();
        if nanos >= 1_000_000_000 {
            // More than or equal to 1 second.
            let secs = duration.as_secs_f64();
            format!("{:.3} sec", secs)
        } else if nanos >= 1_000_000 {
            // More than or equal to 1 millisecond.
            let ms = (nanos as f64) / 1_000_000.0;
            format!("{:.3} ms", ms)
        } else if nanos >= 1_000 {
            // More than or equal to 1 microsecond.
            let micros = (nanos as f64) / 1_000.0;
            format!("{:.3} µs", micros)
        } else {
            // Less than 1 microsecond.
            format!("{} ns", nanos)
        }
    }
    println!("└─ Total build time (init to C code generation): {}", format_duration(total_start.elapsed()));
    
    let compiler_start = Instant::now();
    match linux_b_64(&c_code, &config) {
        Ok(()) => println!("└─ Build succeeded for '{}'.", config.path),
        Err(e) => {
            eprintln!("┌[Error]");
            eprintln!("├─ Build failed for '{}'.", config.path);
            eprintln!("└─ Error: {}", e);
            exit(1);
        }
    }
    println!("Compiler run time: {} ms", compiler_start.elapsed().as_millis());
}

fn main() {
    main_logic();
}
