use std::{
    fs::File,
    io::{Error, Write},
    path::Path,
    process::{exit, Command},
};

use crate::{
    nulibc::{NULIBC, NULIBCH},
    Config,
};

fn create_nulibc_files() -> Result<(), Error> {
    let mut f = File::create("nulibc.c")?;
    f.write_all(NULIBC.as_bytes())?;
    let mut f = File::create("nulibc.h")?;
    f.write_all(NULIBCH.as_bytes())?;
    Ok(())
}

fn translate_target(compiler: &str, target: &str) -> String {
    if compiler.contains("zig") {
        if target.contains("windows") {
            return "x86_64-windows".to_string();
        } else if target.contains("linux") {
            return "x86_64-linux".to_string();
        }
    } else if compiler == "clang" {
        if target.contains("windows") {
            return "x86_64-pc-windows-msvc".to_string();
        } else if target.contains("linux") {
            return "x86_64-linux-gnu".to_string();
        }
    }
    target.to_string()
}

pub fn linux_b_64(code: &String, config: &Config) -> Result<(), Error> {
    let mut cfile = match File::create("_.c") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("┌[Error]");
            eprintln!("├─ Failed to create file: {}", e);
            eprintln!("└─ Please check if you have read/write permissions.");
            exit(1);
        }
    };
    if let Err(e) = cfile.write_all(code.as_bytes()) {
        eprintln!("┌[Error]");
        eprintln!("├─ Failed to write to file: {}", e);
        eprintln!("└─ Please check if you have read/write permissions.");
        exit(1);
    }
    if let Err(e) = create_nulibc_files() {
        eprintln!("┌[Error]");
        eprintln!("├─ Failed to create nulibc files: {}", e);
        eprintln!("└─ Please check if you have read/write permissions.");
        exit(1);
    }
    let mut compiler = String::new();
    if config.cc != "" {
        if config.cc == "zig" {
            #[cfg(target_os = "windows")]
            {
                if Path::new("executables\\zig.exe").exists() {
                    compiler = "executables\\zig.exe".to_string();
                } else if Command::new("zig").arg("--version").output().is_ok() {
                    compiler = "zig.exe".to_string();
                } else {
                    eprintln!("┌[Error]");
                    eprintln!("├─ Specified compiler 'zig' not found.");
                    eprintln!("└─ Exiting build process.");
                    exit(1);
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                if Path::new("executables/zig").exists() {
                    compiler = "executables/zig".to_string();
                } else if Command::new("zig").arg("--version").output().is_ok() {
                    compiler = "zig".to_string();
                } else {
                    eprintln!("┌[Error]");
                    eprintln!("├─ Specified compiler 'zig' not found.");
                    eprintln!("└─ Exiting build process.");
                    exit(1);
                }
            }
        } else if config.cc == "clang" {
            #[cfg(target_os = "windows")]
            {
                if Path::new("executables\\clang.exe").exists() {
                    compiler = "executables\\clang.exe".to_string();
                } else if Command::new("clang").arg("--version").output().is_ok() {
                    compiler = "clang.exe".to_string();
                } else {
                    eprintln!("┌[Error]");
                    eprintln!("├─ Specified compiler 'clang' not found.");
                    eprintln!("└─ Exiting build process.");
                    exit(1);
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                if Path::new("executables/clang").exists() {
                    compiler = "executables/clang".to_string();
                } else if Command::new("clang").arg("--version").output().is_ok() {
                    compiler = "clang".to_string();
                } else {
                    eprintln!("┌[Error]");
                    eprintln!("├─ Specified compiler 'clang' not found.");
                    eprintln!("└─ Exiting build process.");
                    exit(1);
                }
            }
        } else if config.cc == "gcc" {
            #[cfg(target_os = "windows")]
            {
                if Path::new("executables\\gcc.exe").exists() {
                    compiler = "executables\\gcc.exe".to_string();
                } else if Command::new("gcc").arg("--version").output().is_ok() {
                    compiler = "gcc.exe".to_string();
                } else {
                    eprintln!("┌[Error]");
                    eprintln!("├─ Specified compiler 'gcc' not found.");
                    eprintln!("└─ Exiting build process.");
                    exit(1);
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                if Path::new("executables/gcc").exists() {
                    compiler = "executables/gcc".to_string();
                } else if Command::new("gcc").arg("--version").output().is_ok() {
                    compiler = "gcc".to_string();
                } else {
                    eprintln!("┌[Error]");
                    eprintln!("├─ Specified compiler 'gcc' not found.");
                    eprintln!("└─ Exiting build process.");
                    exit(1);
                }
            }
        }
    } else {
        println!("┌[!] No compiler specified with --cc option. Defaulting to clang.");
        #[cfg(target_os = "windows")]
        {
            if Path::new("executables\\clang.exe").exists() {
                compiler = "executables\\clang.exe".to_string();
            } else if Command::new("clang").arg("--version").output().is_ok() {
                compiler = "clang.exe".to_string();
            } else {
                eprintln!("┌[Error]");
                eprintln!("├─ Default compiler 'clang' not found.");
                eprintln!("└─ Exiting build process.");
                exit(1);
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            if Path::new("executables/clang").exists() {
                compiler = "executables/clang".to_string();
            } else if Command::new("clang").arg("--version").output().is_ok() {
                compiler = "clang".to_string();
            } else {
                eprintln!("┌[Error]");
                eprintln!("├─ Default compiler 'clang' not found.");
                eprintln!("└─ Exiting build process.");
                exit(1);
            }
        }
    }
    let host_is_linux = cfg!(target_os = "linux");
    for target in config.targets.iter() {
        let mut local_compiler = compiler.clone();
        if host_is_linux && target.contains("windows") && local_compiler != "zig" {
            if Command::new("x86_64-w64-mingw32-gcc").arg("--version").output().is_ok() {
                local_compiler = "x86_64-w64-mingw32-gcc".to_string();
            }
        }
        let target_arg = translate_target(&local_compiler, target);
        let mut out_file = if config.targets.len() > 1 {
            format!("{}_{}", config.out, target)
        } else {
            config.out.to_string()
        };
        #[cfg(target_os = "windows")]
        {
            if !out_file.ends_with(".exe") {
                out_file.push_str(".exe");
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            if !out_file.ends_with(".out") {
                out_file.push_str(".out");
            }
        }
        println!("[!] Using compiler: {} for target: {}", local_compiler, target);
        let mut cmd = Command::new(&local_compiler);
        if local_compiler.contains("zig") {
            cmd.arg("cc");
            if !target_arg.is_empty() {
                cmd.arg("-target").arg(&target_arg);
            }
            if config.static_flag {
                cmd.arg("-static");
            }
        } else {
            if !target_arg.is_empty() && local_compiler == "clang" {
                cmd.arg("-target").arg(&target_arg);
            }
            if config.static_flag {
                cmd.arg("-static");
            }
        }
        cmd.arg("_.c").arg("nulibc.c").arg("-o").arg(&out_file);
        let output = match cmd.output() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("┌[Error]");
                eprintln!("├─ Failed to execute compiler command: {}", e);
                eprintln!("└─ Exiting build process.");
                exit(1);
            }
        };
        if !output.status.success() {
            eprintln!("┌[Error]");
            eprintln!("├─ Compilation failed for target '{}' using '{}':", target, local_compiler);
            eprintln!("└─ {}", String::from_utf8_lossy(&output.stderr));
            exit(1);
        } else {
            println!("┌[*] Compilation succeeded for target '{}'. Output file: {}", target, out_file);
        }
    }
    Ok(())
}
