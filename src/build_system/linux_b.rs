use std::{
    fs::{self, File},
    io::{Error, Write},
    path::Path,
    process::{exit, Command},
    time::Instant,
};

use crate::{nulibc::{NULIBC, NULIBCH}, Config};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

const SRC_FILE: &str = "_.c";
const NULIBC_C: &str = "nulibc.c";
const NULIBC_H: &str = "nulibc.h";
const HASH_FILE: &str = "hashes";
const COMPILER_KEY: &str = "compiler"; // key used to cache compiler choice

fn compute_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn read_hashes() -> HashMap<String, String> {
    let mut hash_map = HashMap::new();
    if let Ok(file) = File::open(HASH_FILE) {
        for line in BufReader::new(file).lines().flatten() {
            if let Some((file_name, hash)) = line.split_once(' ') {
                hash_map.insert(file_name.to_string(), hash.to_string());
            }
        }
    }
    hash_map
}

fn write_hashes(hashes: &HashMap<String, String>) -> Result<(), Error> {
    let mut file = File::create(HASH_FILE)?;
    for (file_name, hash) in hashes {
        writeln!(file, "{} {}", file_name, hash)?;
    }
    Ok(())
}

fn needs_update(file_path: &str, new_content: &str, hash_map: &HashMap<String, String>) -> bool {
    let new_hash = compute_hash(new_content);
    if let Some(existing_hash) = hash_map.get(file_path) {
        existing_hash != &new_hash
    } else {
        true
    }
}

fn ensure_source_file(code: &str, hash_map: &mut HashMap<String, String>) -> Result<(), Error> {
    if needs_update(SRC_FILE, code, hash_map) {
        println!("[*] Source file '{}' has changed. Updating file.", SRC_FILE);
        File::create(SRC_FILE)?.write_all(code.as_bytes())?;
        let new_hash = compute_hash(code);
        hash_map.insert(SRC_FILE.to_string(), new_hash);
    } else {
        println!("[*] Source file '{}' is up-to-date.", SRC_FILE);
    }
    Ok(())
}

fn create_nulibc_files(hash_map: &mut HashMap<String, String>) -> Result<(), Error> {
    if needs_update(NULIBC_C, NULIBC, hash_map) {
        println!("[*] Nulibc source '{}' has changed. Updating file.", NULIBC_C);
        File::create(NULIBC_C)?.write_all(NULIBC.as_bytes())?;
        let new_hash = compute_hash(NULIBC);
        hash_map.insert(NULIBC_C.to_string(), new_hash);
    } else {
        println!("[*] Nulibc source '{}' is up-to-date.", NULIBC_C);
    }
    if needs_update(NULIBC_H, NULIBCH, hash_map) {
        println!("[*] Nulibc header '{}' has changed. Updating file.", NULIBC_H);
        File::create(NULIBC_H)?.write_all(NULIBCH.as_bytes())?;
        let new_hash = compute_hash(NULIBCH);
        hash_map.insert(NULIBC_H.to_string(), new_hash);
    } else {
        println!("[*] Nulibc header '{}' is up-to-date.", NULIBC_H);
    }
    Ok(())
}

fn needs_recompile(out_file: &str) -> bool {
    let src_meta = fs::metadata(SRC_FILE).ok();
    let nulibc_meta = fs::metadata(NULIBC_C).ok();
    let nulibch_meta = fs::metadata(NULIBC_H).ok();
    let out_meta = fs::metadata(out_file).ok();

    if let Some(out_time) = out_meta.and_then(|m| m.modified().ok()) {
        [src_meta, nulibc_meta, nulibch_meta]
            .iter()
            .flatten()
            .any(|m| m.modified().ok().map_or(true, |t| t > out_time))
    } else {
        true
    }
}

fn translate_target(comp: &str, custom_target: &str) -> Option<String> {
    if custom_target.is_empty() {
        return None;
    }
    if comp.contains("zig") || comp == "clang" {
        if custom_target.contains("windows") {
            if comp.contains("zig") {
                Some("x86_64-windows".into())
            } else {
                Some("x86_64-pc-windows-msvc".into())
            }
        } else if custom_target.contains("linux") {
            if comp.contains("zig") {
                Some("x86_64-linux".into())
            } else {
                Some("x86_64-linux-gnu".into())
            }
        } else {
            Some(custom_target.into())
        }
    } else {
        Some(custom_target.into())
    }
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

/// Build for Linux (64-bit).
pub fn linux_b_64(code: &String, config: &Config) -> Result<(), Error> {
    let overall_start = Instant::now();
    let mut hash_map = read_hashes();

    ensure_source_file(code, &mut hash_map)?;
    create_nulibc_files(&mut hash_map)?;

    // Compiler caching: use key "compiler" in hash_map to store the selected compiler.
    let comp = if let Some(comp_cached) = hash_map.get(COMPILER_KEY) {
        comp_cached.clone()
    } else {
        let comp_found = find_compiler("zig")
            .or_else(|| {
                println!("[!] 'zig' not found, trying 'clang'...");
                find_compiler("clang")
            })
            .or_else(|| {
                println!("[!] 'clang' not found, trying 'gcc'...");
                find_compiler("gcc")
            })
            .unwrap_or_else(|| {
                eprintln!("[X] No suitable compiler found!");
                exit(1)
            });
        hash_map.insert(COMPILER_KEY.to_string(), comp_found.clone());
        comp_found
    };

    println!("[*] Compiler selected: {}", comp);

    // Always use the output filename exactly as specified in config.out.
    let mut out_file = config.out.to_string();
    if Path::new(&out_file).extension().is_none() {
        #[cfg(target_os = "windows")]
        out_file.push_str(".exe");
        #[cfg(not(target_os = "windows"))]
        out_file.push_str(".out");
    }

    if !needs_recompile(&out_file) {
        println!("[*] No changes detected. Reusing existing output file: '{}'", out_file);
        return Ok(());
    }

    let targets = config.targets.clone();
    let static_flag = config.static_flag;
    
    // For each target, compile using the selected compiler.
    // However, the final output file is always named as config.out.
    for target in targets {
        println!("[*] Compiling for target: {}", target);
        let comp_clone = comp.clone();
        let target_arg = translate_target(&comp_clone, &target);
        let static_flag = static_flag;
        let current_out = out_file.clone();
        let compile_start = Instant::now();
        let mut cmd = Command::new(&comp_clone);
        if comp_clone.contains("zig") {
            cmd.arg("cc");
            if let Some(targ) = target_arg {
                cmd.args(&["-target", &targ]);
            }
            if static_flag {
                cmd.arg("-static");
            }
            cmd.args(&["-pipe", "-flto"]);
        } else if comp_clone.contains("clang") {
            if let Some(targ) = target_arg {
                cmd.args(&["-target", &targ]);
            }
            if static_flag {
                cmd.arg("-static");
            }
            cmd.args(&["-pipe", "-flto"]);
        } else {
            // For any other compiler (e.g., gcc), do not add optimization flags.
            if static_flag {
                cmd.arg("-static");
            }
        }
        cmd.args(&[SRC_FILE, NULIBC_C, "-o", &current_out]);
        let output = cmd.output().unwrap_or_else(|e| {
            eprintln!("[X] Compile error: {}", e);
            exit(1)
        });
        let compile_time = compile_start.elapsed().as_millis();
        if !output.status.success() {
            eprintln!("[X] Failed {}: {}", target, String::from_utf8_lossy(&output.stderr));
            exit(1);
        } else {
            println!("[*] Success {}: {} ({} ms)", target, current_out, compile_time);
        }
    }
    
    if let Err(e) = write_hashes(&hash_map) {
        eprintln!("[X] Error writing hash file: {}", e);
    }

    let overall_time = overall_start.elapsed().as_millis();
    println!("[*] Total compilation time: {} ms", overall_time);
    Ok(())
}
