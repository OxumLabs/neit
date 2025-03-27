use std::{
    env::args,
    fs::File,
    io::{Read, BufRead, BufReader},
    path::Path,
    process::exit,
    time::{Duration, Instant},
};
use std::collections::HashMap;
use build_system::linux_b::linux_b_64;
use c_gens::makec::make_c;
use parse_systems::parse;
use tok_system::{lexer::LexicalAnalysis, tokens::Token};
use colored::*;

pub mod build_system;
pub mod c_gens;
pub mod err_system;
pub mod nulibc;
pub mod parse_systems;
pub mod helpers;
pub mod tok_system;
pub mod optimisers;

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
        "linux-x86-64" | "linx8664"   => "linux-x86-64",
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
            targets = value.split(',').map(|s| normalize_target(s)).collect();
        } else if arg_static.starts_with("--cc=") {
            cc = Box::leak(arg_static["--cc=".len()..].to_string().into_boxed_str());
        } else {
            println!("{}", format!("┌[Warning] Unknown option '{}'", arg_static).yellow());
        }
    }
    Config { command, path, static_flag, out, targets, cc }
}

fn print_help() {
    println!("{}", "┌[*] Neit Programming Language - Help".blue());
    println!("{}", "├─ Usage: neit <command> <file/folder> <options>".blue());
    println!("{}", "├─ Commands:".blue());
    println!("{}", "│   ├─ build   - Build a Neit source file/folder".blue());
    println!("{}", "│   └─ help    - Display this help message".blue());
    println!("{}", "├─ Options:".blue());
    println!("{}", "│   ├─ --static                - Build with static linking".blue());
    println!("{}", "│   ├─ --out=<file_name>       - Specify output file name (default: out)".blue());
    println!("{}", "│   ├─ --target=<target>       - Specify target platform(s) (default: host OS)".blue());
    println!("{}", "│   │        Acceptable targets for Windows: \"windows-x86-64\" or \"winx8664\"".blue());
    println!("{}", "│   │        Acceptable targets for Linux:   \"linux-x86-64\" or \"linx8664\"".blue());
    println!("{}", "│   │        Multiple targets can be separated by commas.".blue());
    println!("{}", "│   └─ --cc=<compiler>         - Specify compiler (zig, clang, gcc)".blue());
    println!("{}", "└─ Example: neit build ./source.neit --out=program --target=linux-x86-64,winx8664 --cc=zig".blue());
}

use sha2::{Sha256, Digest};

fn compute_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn read_hashes() -> HashMap<String, String> {
    let mut hash_map = HashMap::new();
    if let Ok(file) = File::open("hashes") {
        for line in BufReader::new(file).lines().flatten() {
            if let Some((file_name, hash)) = line.split_once(' ') {
                hash_map.insert(file_name.to_string(), hash.to_string());
            }
        }
    }
    hash_map
}

fn source_has_changed(path: &str, hash_map: &mut HashMap<String, String>) -> bool {
    let mut content = String::new();
    if let Ok(mut file) = File::open(path) {
        if file.read_to_string(&mut content).is_err() {
            return true;
        }
    } else {
        return true;
    }
    let new_hash = compute_hash(&content);
    if let Some(existing_hash) = hash_map.get(path) {
        if existing_hash == &new_hash {
            return false;
        }
    }
    hash_map.insert(path.to_string(), new_hash);
    true
}

fn format_duration(duration: Duration) -> String {
    let nanos = duration.as_nanos();
    if nanos >= 1_000_000_000 {
        let secs = duration.as_secs_f64();
        format!("{:.3} sec", secs)
    } else if nanos >= 1_000_000 {
        let ms = (nanos as f64) / 1_000_000.0;
        format!("{:.3} ms", ms)
    } else if nanos >= 1_000 {
        let micros = (nanos as f64) / 1_000.0;
        format!("{:.3} µs", micros)
    } else {
        format!("{} ns", nanos)
    }
}

fn main_logic() {
    let total_start = Instant::now();
    let config = parse_config();
    if config.command == "help" {
        print_help();
        exit(0);
    }
    if config.command != "build" && config.command != "neit" {
        eprintln!("{}", format!("┌[!!] CRITICAL ERROR").red());
        eprintln!("{}", format!("├─ Command '{}' is not recognized.", config.command).red());
        eprintln!("{}", "└─ Please refer to 'neit help' for a list of valid commands.".red());
        exit(1);
    }
    println!("{}", "┌[*] Neit Build System - Initiating Build Process".blue());
    println!("{}", format!("├─ Compiling source file: '{}'", config.path).cyan());
    let proj = Path::new(config.path);
    if !proj.exists() {
        eprintln!("{}", format!("┌[Error] File/Directory Not Found").red());
        eprintln!("{}", format!("├─ The path '{}' does not exist.", config.path).red());
        eprintln!("{}", "└─ Verify the path and try again.".red());
        exit(1);
    }
    if proj.is_dir() {
        eprintln!("{}", "┌[Error] Invalid Input: Directory Provided".red());
        eprintln!("{}", "├─ Only source files are supported.".red());
        eprintln!("{}", "└─ Please provide a valid file path.".red());
        exit(1);
    }
    let mut file = File::open(config.path).unwrap_or_else(|e| {
        eprintln!("{}", "┌[Error] Unable to Open Source File".red());
        eprintln!("{}", format!("├─ Failed to open '{}': {}", config.path, e).red());
        eprintln!("{}", "└─ Check file permissions and try again.".red());
        exit(1);
    });
    let mut code = String::new();
    if file.read_to_string(&mut code).is_err() {
        eprintln!("{}", "┌[Error] File Read Failure".red());
        eprintln!("{}", format!("├─ Unable to read '{}'.", config.path).red());
        eprintln!("{}", "└─ Ensure the file is not corrupted and is accessible.".red());
        exit(1);
    }
//let topcode = code.clone();
    println!("{}", format!("├─ Source file '{}' loaded successfully.", config.path).cyan());
    let mut hash_map = read_hashes();
    if !source_has_changed(config.path, &mut hash_map) {
        let mut out_file = config.out.to_string();
        if Path::new(&out_file).extension().is_none() {
            #[cfg(target_os = "windows")]
            out_file.push_str(".exe");
            #[cfg(not(target_os = "windows"))]
            out_file.push_str(".out");
        }
        println!("{}", "[*] No changes detected in the source.".cyan());
        println!("{}", "[*] Skipping re-tokenization and parsing.".cyan());
        println!("{}", format!("└─ Reusing existing output file: '{}'", out_file).cyan());
        exit(0);
    } else {
        println!("{}", "[*] Source modifications detected.".cyan());
        println!("{}", "[*] Tokenizing source code...".cyan());
        let mut tokens: Vec<Token> = Vec::new();
        tokens.run_lexical_analysis(&code);
        println!("{}", format!("[*] Tokenization complete ({} tokens produced).", tokens.len()).cyan());
        let proj_path: &'static str = Box::leak(proj.display().to_string().into_boxed_str());
        let mut collected_vars = Vec::new();
        let mut collected_errors = Vec::new();
        let (ast, _, _) = parse(&tokens, &code, proj_path, false, &mut collected_vars, &mut collected_errors,1);
        println!("{}", "[*] Parsing complete. AST generated successfully.".cyan());
        let mut math_exprs = HashMap::new();
        code = make_c(&ast, true, &mut collected_vars, &mut collected_errors, &mut math_exprs);
        println!("{}", "[*] Intermediate C code generated.".cyan());
        println!("{}", format!("└─ Build preparation completed in {}.", format_duration(total_start.elapsed())).cyan());
    }
    let compiler_start = Instant::now();
    match linux_b_64(&code, &config) {
        Ok(()) => println!("{}", format!("└─ Build SUCCESS: Output generated for '{}' , output file is named '{}'", config.path,config.out).green()),
        Err(e) => {
            eprintln!("{}", "┌[Error] Build FAILURE".red());
            eprintln!("{}", format!("├─ Compilation failed for '{}'.", config.path).red());
            eprintln!("{}", format!("└─ Compiler error details: {}", e).red());
            exit(1);
        }
    }
    println!("{}", format!("└─ Compiler execution time: {} ms", compiler_start.elapsed().as_millis()).magenta());
}

fn main() {
    main_logic();
}
