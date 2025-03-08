use std::{
    fs::File,
    io::{Error, Write},
    path::Path,
    process::{exit, Command},
    thread,
    time::Instant,
};

use crate::{nulibc::{NULIBC, NULIBCH}, Config};

const SRC_FILE: &str = "_.c";
const NULIBC_C: &str = "nulibc.c";
const NULIBC_H: &str = "nulibc.h";

fn create_nulibc_files() -> Result<(), Error> {
    File::create(NULIBC_C)?.write_all(NULIBC.as_bytes())?;
    File::create(NULIBC_H)?.write_all(NULIBCH.as_bytes())?;
    Ok(())
}

fn translate_target(comp: &str, target: &str) -> String {
    if comp.contains("zig") {
        if target.contains("windows") { "x86_64-windows".into() }
        else if target.contains("linux") { "x86_64-linux".into() }
        else { target.into() }
    } else if comp == "clang" {
        if target.contains("windows") { "x86_64-pc-windows-msvc".into() }
        else if target.contains("linux") { "x86_64-linux-gnu".into() }
        else { target.into() }
    } else { target.into() }
}

fn find_compiler(compiler: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    let exe_suffix = ".exe";
    #[cfg(not(target_os = "windows"))]
    let exe_suffix = "";
    
    let local_path = format!("executables/{}{}", compiler, exe_suffix);
    if Path::new(&local_path).exists() {
        return Some(local_path);
    }
    if Command::new(compiler).arg("--version").output().is_ok() {
        return Some(compiler.to_string());
    }
    None
}

pub fn linux_b_64(code: &String, config: &Config) -> Result<(), Error> {
    let overall_start = Instant::now();

    File::create(SRC_FILE)?.write_all(code.as_bytes())?;
    create_nulibc_files()?;
    let compiler_key = if config.cc.is_empty() { "clang" } else { config.cc };
    let comp = find_compiler(compiler_key).unwrap_or_else(|| {
        eprintln!("{} not found", compiler_key);
        exit(1);
    });
    let host_is_linux = cfg!(target_os = "linux");
    let mingw_available = host_is_linux && Command::new("x86_64-w64-mingw32-gcc")
        .arg("--version").output().is_ok();
    let targets = config.targets.clone();
    let out_base = config.out;
    let static_flag = config.static_flag;
    let multiple_targets = config.targets.len() > 1;
    
    let mut handles = Vec::new();
    for target in targets {
        let comp_clone = comp.clone();
        let lcomp = if host_is_linux && target.contains("windows") && comp_clone != "zig" && mingw_available {
            "x86_64-w64-mingw32-gcc".to_string()
        } else {
            comp_clone
        };
        let target_arg = translate_target(&lcomp, &target);
        let mut out_file = if multiple_targets {
            format!("{}_{}", out_base, target)
        } else {
            out_base.to_string()
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
        println!("┌[*] Using {} for target {}", lcomp, target);
        let handle = thread::spawn(move || {
            let compile_start = Instant::now();
            let mut cmd = Command::new(&lcomp);
            if lcomp.contains("zig") {
                cmd.arg("cc");
                if !target_arg.is_empty() {
                    cmd.args(&["-target", &target_arg]);
                }
                if static_flag {
                    cmd.arg("-static");
                }
            } else {
                if !target_arg.is_empty() && lcomp == "clang" {
                    cmd.args(&["-target", &target_arg]);
                }
                if static_flag {
                    cmd.arg("-static");
                }
            }
            cmd.args(&[SRC_FILE, NULIBC_C, "-o"]).arg(&out_file);
            let output = cmd.output().unwrap_or_else(|e| {
                eprintln!("Compile error: {}", e);
                exit(1);
            });
            let compile_time = compile_start.elapsed().as_millis();
            if !output.status.success() {
                eprintln!("Failed {}: {}", target, String::from_utf8_lossy(&output.stderr));
                exit(1);
            } else {
                println!("├─ Succeeded {}: {} ({} ms)", target, out_file, compile_time);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Compilation thread panicked");
    }
    let overall_time = overall_start.elapsed().as_millis();
    println!("└─ Total compilation time: {} ms", overall_time);
    Ok(())
}
