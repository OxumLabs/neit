use std::{
    fs::{self, File},
    io::{self, stdin, Write},
    path::Path,
    process::{exit, Command},
};

pub fn compile(asm: &String, proj: &str, target: &str, project_name: &str) {
    println!("Target at Compile (9) : {}", target);

    let build_dir = Path::new(proj).join("build");
    if !build_dir.exists() {
        fs::create_dir_all(&build_dir).expect("✘ Yikes! Failed to create build directory.");
    }

    let output_file = match target {
        "linux" | "lin_asm" => build_dir.join(project_name),
        "windows" | "win_asm" => build_dir.join(format!("{}.exe", project_name)),
        "C" => build_dir.join(format!("{}.c", project_name)),
        _ => {
            eprintln!("✘ Unsupported build target '{}'.", target);
            eprintln!("→ Hint: Check the target options.");
            eprintln!("⚙ [Location: compile function]");
            exit(1);
        }
    };

    let asm_file_path = "./temp.asm";
    match File::create(asm_file_path) {
        Ok(mut asmf) => match asmf.write_all(asm.as_bytes()) {
            Ok(_) => {
                // Platform-specific compilation logic
                match target {
                    // Linux Assembly Compilation
                    "linux" | "lin_asm" => compile_linux(&asm_file_path, &output_file),

                    // Windows Assembly Compilation
                    "windows" | "win_asm" => compile_windows(&asm_file_path, &output_file),

                    _ => {
                        eprintln!("✘ Unsupported target '{}'.", target);
                        eprintln!("→ Hint: Check the target options.");
                        eprintln!("⚙ [Location: compile function]");
                        exit(1);
                    }
                }

                //Clean up temporary files
                fs::remove_file(asm_file_path).expect("Failed to delete temporary ASM file");
                if target == "linux" {
                    fs::remove_file("temp.o").expect("Failed to delete temporary object file");
                } else if target == "windows" {
                    fs::remove_file("temp.obj").expect("Failed to delete temporary object file");
                }
            }
            Err(_) => {
                eprintln!("✘ Unable to write assembly code to file.");
                eprintln!("→ Hint: Ensure correct permissions.");
                eprintln!("⚙ [Location: compile function]");
                exit(1);
            }
        },
        Err(_) => {
            eprintln!("✘ Unable to create assembly file.");
            eprintln!("→ Hint: Ensure correct permissions.");
            eprintln!("⚙ [Location: compile function]");
            exit(1);
        }
    }
}

fn compile_linux(asm_file_path: &str, output_file: &Path) {
    let nasm_args = vec!["-f", "elf64", "-o", "temp.o", asm_file_path];
    let status = Command::new("nasm")
        .args(nasm_args)
        .status()
        .expect("✘ Failed to execute `nasm` command");

    if !status.success() {
        eprintln!("✘ Assembly for Linux failed.");
        eprintln!("→ Hint: Check NASM installation and syntax.");
        eprintln!("⚙ [Location: compile_linux function]");
        return;
    }

    let status = Command::new("clang")
        .arg("-o")
        .arg(output_file)
        .arg("temp.o")
        .arg("-nostdlib")
        .status()
        .expect("✘ Failed to execute `clang` command");

    if !status.success() {
        eprintln!("✘ Linking for Linux failed.");
        eprintln!("→ Hint: Ensure Clang is correctly set up.");
        eprintln!("⚙ [Location: compile_linux function]");
    } else {
        println!("Successfully built for Linux: {:?}", output_file);
    }
}

fn compile_windows(asm_file_path: &str, output_file: &Path) {
    let nasm_args = vec!["-f", "win64", "-o", "temp.obj", asm_file_path];
    let status = Command::new("nasm")
        .args(nasm_args)
        .status()
        .expect("✘ Failed to execute `nasm` command");

    if !status.success() {
        eprintln!("✘ Assembly for Windows failed.");
        eprintln!("→ Hint: Check NASM installation and syntax.");
        eprintln!("⚙ [Location: compile_windows function]");
        return; // Exit early if assembly fails
    }

    let status = Command::new("clang")
        .arg("-o")
        .arg(output_file)
        .arg("temp.obj")
        .arg("-static")
        .arg("-lkernel32")
        .arg("-Wl,/ENTRY:main") // Set entry point to `main`
        .arg("-Wl,/LARGEADDRESSAWARE:NO")
        .status()
        .expect("✘ Failed to execute `clang` command");

    if !status.success() {
        eprintln!("✘ Linking for Windows failed.");
        eprintln!("→ Hint: Ensure Clang is correctly set up.");
        eprintln!("⚙ [Location: compile_windows function]");
    } else {
        println!("Successfully built for Windows: {:?}", output_file);
    }
}

pub fn check_tools_installed() -> io::Result<()> {
    // if !is_tool_installed("nasm") {
    //     prompt_install("nasm")?;
    // }
    if !is_tool_installed("clang") {
        prompt_install("clang")?;
    }
    Ok(())
}

fn is_tool_installed(tool: &str) -> bool {
    Command::new(tool).output().is_ok()
}

fn prompt_install(tool: &str) -> io::Result<()> {
    let os_type = std::env::consts::OS;

    println!("✘ {} has been hidding pretty well ,  I am not able to find it\nCan you make sure It is installed and on system path?", tool);

    if tool == "nasm" {
        if os_type == "windows" {
            println!(
                "→ Hint: To install NASM on Windows, visit: https://www.nasm.us/pub/nasm/releasebuilds/"
            );
        } else if os_type == "linux" {
            println!("→ Hint: For Linux: sudo apt install nasm (for Ubuntu/Debian) or similar for other distros.");
        }
    } else if tool == "clang" {
        if os_type == "windows" {
            println!(
                "→ Hint: To install Clang on Windows, visit: https://github.com/llvm/llvm-project/releases"
            );
        } else if os_type == "linux" {
            println!(
                "→ Hint: For Linux: sudo apt install clang lld (for Ubuntu/Debian) or similar."
            );
        }
    }

    println!("\nPress ENTER to exit.");
    let mut i = String::new();
    stdin().read_line(&mut i).unwrap();
    exit(1);
}
