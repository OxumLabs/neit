use std::{
    fs::{self, File},
    io::{self, stdin, Write},
    path::Path,
    process::{exit, Command},
};

pub fn compile(asm: &String, proj: &str, target: &str, project_name: &str) {
    let _cti = check_tools_installed().is_ok();

    let build_dir = Path::new(proj).join("build");
    if !build_dir.exists() {
        fs::create_dir_all(&build_dir).expect("Failed to create build directory");
    }

    let output_file = match target {
        "linux" => build_dir.join(project_name),
        "windows" => build_dir.join(format!("{}.exe", project_name)),
        _ => {
            eprintln!("Error: Unsupported build target '{}'.", target);
            exit(1);
        }
    };

    let asm_file_path = "./temp.asm";
    match File::create(asm_file_path) {
        Ok(mut asmf) => match asmf.write_all(asm.as_bytes()) {
            Ok(_) => {
                match target {
                    "linux" => {
                        let nasm_args = vec!["-f", "elf64", "-o", "temp.o", asm_file_path];
                        let status = Command::new("nasm")
                            .args(nasm_args)
                            .stdout(std::process::Stdio::null()) // Suppress output
                            .stderr(std::process::Stdio::null()) // Suppress error output
                            .status()
                            .expect("Failed to execute `nasm` command");

                        if !status.success() {
                            eprintln!("Error: Assembly for Linux failed");
                        } else {
                            let status = Command::new("clang")
                                .arg("-o")
                                .arg(&output_file)
                                .arg("temp.o")
                                .arg("-nostdlib")
                                .arg("-Wl,--no-relax") // Optional: suppress warnings
                                .stdout(std::process::Stdio::null()) // Suppress output
                                .stderr(std::process::Stdio::null()) // Suppress error output
                                .status()
                                .expect("Failed to execute `clang` command");

                            if !status.success() {
                                eprintln!("Error: Linking for Linux failed");
                            } else {
                                println!("Successfully built for {}: {:?}", target, output_file);
                            }
                        }
                    }
                    "windows" => {
                        let nasm_args = vec!["-f", "win64", "-o", "temp.obj", asm_file_path];
                        let status = Command::new("nasm")
                            .args(nasm_args)
                            .stdout(std::process::Stdio::null()) // Suppress output
                            .stderr(std::process::Stdio::null()) // Suppress error output
                            .status()
                            .expect("Failed to execute `nasm` command");

                        if !status.success() {
                            eprintln!("Error: Assembly for Windows failed");
                        } else {
                            let status = Command::new("clang")
                                .arg("-o")
                                .arg(&output_file)
                                .arg("temp.obj")
                                .arg("-nostdlib")
                                .arg("-Wl,--no-relax") // Optional: suppress warnings
                                .stdout(std::process::Stdio::null()) // Suppress output
                                .stderr(std::process::Stdio::null()) // Suppress error output
                                .status()
                                .expect("Failed to execute `clang` command");

                            if !status.success() {
                                eprintln!("Error: Linking for Windows failed");
                            } else {
                                println!("Successfully built for {}: {:?}", target, output_file);
                            }
                        }
                    }
                    _ => {
                        eprintln!("Error: Unsupported target '{}'.", target);
                        exit(1);
                    }
                }

                // Clean up temporary files
                fs::remove_file(asm_file_path).expect("Failed to delete temporary ASM file");
                if target == "linux" {
                    fs::remove_file("temp.o").expect("Failed to delete temporary object file");
                } else if target == "windows" {
                    fs::remove_file("temp.obj").expect("Failed to delete temporary object file");
                }

                // Do not exit after successful build
            }
            Err(_) => {
                eprintln!("Error: Unable to write assembly code to file\nHint: Ensure correct permissions");
                exit(1);
            }
        },
        Err(_) => {
            eprintln!("Error: Unable to create assembly file\nHint: Ensure correct permissions");
            exit(1);
        }
    }
}

pub fn check_tools_installed() -> io::Result<()> {
    if !is_tool_installed("nasm") {
        prompt_install("nasm")?;
    }
    if !is_tool_installed("clang") {
        prompt_install("clang")?;
    }
    Ok(())
}

fn is_tool_installed(tool: &str) -> bool {
    Command::new(tool).arg("--version").output().is_ok()
}

fn prompt_install(tool: &str) -> io::Result<()> {
    let os_type = std::env::consts::OS;

    println!("Error: {} is not installed.", tool);

    if tool == "nasm" {
        if os_type == "windows" {
            println!("To install nasm on Windows, visit:");
            println!("  https://www.nasm.us/pub/nasm/releasebuilds/");
            println!("Scroll to the bottom and find the folder from the last release that doesn't contain 'rc*'.");
            println!("Go into that folder and download the Windows installer for nasm.");
            println!("Once downloaded, run the installer to install nasm.");
            println!("\n\n*PRESS ENTER TO EXIT**");
            let mut i = String::new();
            stdin().read_line(&mut i).unwrap();
        } else if os_type == "linux" {
            println!("To install nasm on Linux, please look up how to install nasm for your distribution.");
            println!("Common commands are:");
            println!("  - For Ubuntu/Debian: sudo apt install nasm");
            println!("  - For Fedora: sudo dnf install nasm");
            println!("  - For Arch: sudo pacman -S nasm");
            println!("More installation commands can be found at : https://github.com/OxumLabs/neit?tab=readme-ov-file#linux-installation");

            println!("\n\n*PRESS ENTER TO EXIT**");
            let mut i = String::new();
            stdin().read_line(&mut i).unwrap();
        }
    } else if tool == "clang" {
        if os_type == "windows" {
            println!("To install clang on Windows, follow these steps:");
            println!("1. Go to the release page: https://github.com/llvm/llvm-project/releases");
            println!("2. Download the latest LLVM installer for Win64 (look for a file named something like LLVM-19.1.0-win64.exe; the version number may differ).");
            println!("3. Run the installer and follow the setup instructions to complete the installation.");

            println!("\n\n*PRESS ENTER TO EXIT**");
            let mut i = String::new();
            stdin().read_line(&mut i).unwrap();
        } else if os_type == "linux" {
            println!("To install clang on Linux, please look up how to install clang for your distribution.");
            println!("Common commands are:");
            println!("  - For Ubuntu/Debian: sudo apt install clang");
            println!("  - For Fedora: sudo dnf install clang");
            println!("  - For Arch: sudo pacman -S clang");

            println!("\n\n*PRESS ENTER TO EXIT**");
            let mut i = String::new();
            stdin().read_line(&mut i).unwrap();
        }
    }
    Ok(())
}
