use std::{
    env::{consts::OS, current_exe},
    fs::{self, File},
    io::Write,
    path::Path,
    process::{exit, Command},
};

use colored::Colorize;

use crate::{
    codegen::codegen,
    grm,
    lex::{lex, Tokens},
    nulibc,
    p::parse,
};

pub fn build(args: &[String]) {
    let exe_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting the current executable path: {}", e);
            std::process::exit(1);
        }
    };
    
    let exe_dir = match exe_path.parent() {
        Some(dir) => dir,
        None => {
            eprintln!("Error getting the directory of the executable.");
            std::process::exit(1);
        }
    };
    
    let source_path = exe_dir.join("libnulibc.a");
    
    if !source_path.exists() {
        eprintln!("Source file does not exist: {:?}", source_path);
        std::process::exit(1);
    }
    
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting the current working directory: {}", e);
            std::process::exit(1);
        }
    };
    
    let destination_path = current_dir.join("libnulibc.a");
    
    if let Err(e) = std::fs::copy(&source_path, &destination_path) {
        eprintln!("Error copying file from {:?} to {:?}: {}", source_path, destination_path, e);
        std::process::exit(1);
    }
    
    let src_path = Path::new(&args[2]);

    if !src_path.exists() {
        eprintln!(
            "{} {}",
            "Error :~ Cannot stat source file/project :".red(),
            src_path.display()
        );
        exit(-1);
    }

    if src_path.is_dir() {
        build_dir(&args[2..], src_path);
    } else if src_path.is_file() {
        build_file(&args[2..], src_path);
    } else {
        eprintln!(
            "{} {}",
            "Error :~ Invalid source path :".red(),
            src_path.display()
        );
        exit(-1);
    }
}
#[allow(unused)]
fn build_dir(args: &[String], src: &Path) {
    println!("{}", "Building projects coming soon!".red());
    exit(0);
}

fn build_file(args: &[String], src: &Path) {
    println!("{} {}", "Building file :".green(), src.display());

    let mut code = fs::read_to_string(src).unwrap_or_else(|e| {
        eprintln!(
            "{} {}",
            "Error :~ Cannot read source file :".red(),
            src.display()
        );
        eprintln!("{} {}", "Error MSG :~".red(), e.to_string().bright_red());
        exit(-1);
    });
    for arg in args.iter() {
        match arg {
            arg if arg.starts_with("-g=") || arg.starts_with("--grammar=") => {
                let grmf = if arg.starts_with("-g=") {
                    arg.trim_start_matches("-g=")
                } else {
                    arg.trim_start_matches("--grammar=")
                };
                grm::pgrm(&mut code, grmf);
                break;
            }
            _ => {}
        }
    }

    println!("{}", "Lexing file...".green());
    let mut toks = Tokens::new();
    lex(&code, &mut toks);
    //println!("[DEBUG] toks : {:?}", toks);

    println!("{}", "Parsing file...".green());
    let mut nst = parse(
        &toks,
        &code.split("\n").collect::<Vec<&str>>(),
        src.display().to_string().as_str(),
        true,
        &mut Vec::new(),
    );
    //println!("[DEBUG] nst : {:?}", nst);

    println!("{}", "Parsing CLI arguments...".green());
    let _target_os = parse_target_os(args);
    let opt_level = parse_optimization(args);
    let output_file = parse_output(args).trim().to_string();

    println!("{}", "Generating code...".green());
    let ccode = codegen(&mut nst, true, true, true);

    println!("{}", "Writing C code to file...".green());
    write_to_file(&ccode, &output_file);

    println!("{}", "Generating Clang command...".green());
    let cmd = build_clang_command(args, &output_file, src, opt_level);

    println!("{}", "Running Clang command...".green());
    run_clang_command(cmd, &output_file, args);
}

fn parse_target_os(args: &[String]) -> String {
    for arg in args {
        if let Some(target) = parse_flag(arg, "-t=", "--target=") {
            return target;
        }
    }

    match OS {
        "linux" => "linux".to_string(),
        "windows" => "windows".to_string(),
        "macos" => "macos".to_string(),
        _ => {
            eprintln!(
                "{}\n{}",
                "Error :~ Cannot detect target OS.".red(),
                "Please specify target OS using '--target=<OS_NAME>'".bright_red()
            );
            exit(-1);
        }
    }
}

fn parse_optimization(args: &[String]) -> i32 {
    for arg in args {
        if let Some(opt) = parse_flag(arg, "-opt=", "--optimisation=") {
            return opt.parse::<i32>().unwrap_or_else(|_| {
                eprintln!("{}", "The optimization level is invalid.".red());
                exit(-1);
            });
        }
    }
    -1 // Default optimization level
}

fn parse_output(args: &[String]) -> String {
    for arg in args {
        if let Some(output) = parse_flag(arg, "-o=", "--out=") {
            return output;
        }
    }
    "output".to_string() // Default output file name
}

fn parse_flag(arg: &str, flag1: &str, flag2: &str) -> Option<String> {
    if arg.starts_with(flag1) {
        Some(arg.split('=').collect::<Vec<&str>>()[1].to_string())
    } else if arg.starts_with(flag2) {
        Some(arg.split('=').collect::<Vec<&str>>()[1].to_string())
    } else {
        None
    }
}

fn write_to_file(ccode: &str, output_file: &str) {
    match File::create(format!("{}.c", output_file)) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(ccode.as_bytes()) {
                eprintln!(
                    "{} {}",
                    "Error :~ Cannot write to file :".red(),
                    output_file
                );
                eprintln!("{} {}", "Error MSG :~".red(), e.to_string().bright_red());
                exit(-1);
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Error :~ Cannot create file :".red(), output_file);
            eprintln!("{} {}", "Error MSG :~".red(), e.to_string().bright_red());
            exit(-1);
        }
    }
}

fn build_clang_command(args: &[String], output_file: &str, _src: &Path, opt_level: i32) -> Command {
    let mut cmd = Command::new("clang");
    cmd.arg(format!("{}.c", output_file));
    cmd.arg("-I.");
    cmd.arg("-static");
    cmd.arg("-Wno-return-type");
    let nulibcp = Path::new("nulibc.c");
    let nulibchp = Path::new("nulibc.h");
    match File::create(nulibcp) {
        Ok(mut f) => {
            f.write_all(nulibc::NULIBC.as_bytes()).unwrap();
            cmd.arg("nulibc.c");
        }
        Err(e) => {
            eprintln!("{}{}", "Error :~ Unable to create nulibc.c file :~ {}", e);
            exit(1);
        }
    }
    match File::create(nulibchp) {
        Ok(mut f) => {
            f.write_all(nulibc::NULIBCH.as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("{}{}", "Error :~ Unable to create nulibc.c file :~ {}", e);
            exit(1);
        }
    }
    cmd.arg(format!("-o{}", output_file));

    if args.contains(&"-static".to_string()) {
        cmd.arg("-static");
    }

    let opt_flags = get_optimization_flags(opt_level);
    cmd.args(opt_flags);

    //println!("[DEBUG] cmd: {:?}", cmd);
    cmd
}

fn get_optimization_flags(level: i32) -> Vec<&'static str> {
    match level {
        1 => vec!["-O1", "-fno-inline-small-functions", "-funroll-loops"],
        2 => vec![
            "-O2",
            "-fstrict-aliasing",
            "-fomit-frame-pointer",
            "-funroll-loops",
            "-flto",
        ],
        3 => vec![
            "-O3",
            "-funroll-loops",
            "-ftree-vectorize",
            "-flto",
            "-finline-functions",
        ],
        4 => vec![
            "-O3",
            "-funroll-loops",
            "-ftree-vectorize",
            "-flto",
            "-finline-functions",
            "-fomit-frame-pointer",
            "-fstrict-aliasing",
            "-march=native",
            "-funroll-loops",
            "-ftree-slp-vectorize",
            "-foptimize-sibling-calls",
            "-fstack-protector-strong",
            "-ffast-math",
            "-fno-strict-aliasing",
            "-funroll-loops",
            "-fvisibility=hidden",
            "-fvisibility-inlines-hidden",
            "-ffunction-sections",
            "-fdata-sections",
            "-flto=full",
            "-ftree-vectorize",
            "-fno-math-errno",
            "-funsafe-math-optimizations",
            "-fomit-frame-pointer",
            "-falign-functions=32",
            "-fno-math-errno",
            "-fno-rounding-math",
            "-ftree-vectorize",
            "-fvisibility=hidden",
            "-fno-strict-aliasing",
            "-ffast-math",
            "-ftree-vectorize",
            "-fno-common",
            "-fomit-frame-pointer",
            "-fPIC",
            "-fno-inline",
        ],
        _ => vec![],
    }
}

fn run_clang_command(mut cmd: Command, output_file: &str, args: &[String]) {
    // println!("[DEBUG] cmd: {:?}", cmd);

    match cmd.status() {
        Ok(status) => {
            if status.success() {
                println!(
                    "{}",
                    "Neit-2-C Converted Code compiled successfully!".green()
                );
                if !args.contains(&"-rc".to_string()) && !args.contains(&"--retian-c".to_string()) {
                    match fs::remove_file(format!("{}.c", output_file)) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{} {}", "Error :~ Cannot remove file :".red(), output_file);
                            eprintln!("{} {}", "Error MSG :~".red(), e.to_string().bright_red());
                            exit(-1);
                        }
                    }
                    match fs::remove_file(format!("nulibc.c")) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{}", "Error :~ Cannot remove file : nulibc.c".red());
                            eprintln!("{} {}", "Error MSG :~".red(), e.to_string().bright_red());
                            exit(-1);
                        }
                    }
                    match fs::remove_file(format!("nulibc.h")) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{}", "Error :~ Cannot remove file : nulibc.h".red());
                            eprintln!("{} {}", "Error MSG :~".red(), e.to_string().bright_red());
                            exit(-1);
                        }
                    }
                }
            } else {
                eprintln!("{}", "Clang compilation failed.".red());
                exit(-1);
            }
        }
        Err(e) => {
            eprintln!(
                "{} {}",
                "Error :~ Cannot run Clang command :".red(),
                e.to_string().bright_red()
            );
            exit(-1);
        }
    }
}
