use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{exit, Command},
};

pub fn compile(asm: &String, proj: &str, target: &str, project_name: &str) {
    // Create the build directory if it doesn't exist
    let build_dir = Path::new(proj).join("build");
    if !build_dir.exists() {
        fs::create_dir_all(&build_dir).expect("Failed to create build directory");
    }

    // Determine output file names based on the target
    let output_file = match target {
        "linux" => build_dir.join(project_name), // For Linux, output an executable
        "windows" => build_dir.join(format!("{}.exe", project_name)), // For Windows, output an .exe file
        _ => {
            eprintln!("Error: Unsupported build target '{}'.", target);
            exit(1);
        }
    };

    // Create the temporary ASM file
    let asm_file_path = "./temp.asm";
    match File::create(asm_file_path) {
        Ok(mut asmf) => match asmf.write_all(asm.as_bytes()) {
            Ok(_) => {
                // Build based on the target
                match target {
                    "linux" => {
                        // Assemble for Linux
                        let nasm_args = vec!["-f", "elf64", "-o", "temp.o", asm_file_path];
                        let status = Command::new("nasm")
                            .args(nasm_args)
                            .status()
                            .expect("Failed to execute `nasm` command");

                        if !status.success() {
                            eprintln!("Error: Assembly for Linux failed");
                            exit(1);
                        }

                        // Link the object file for Linux
                        let status = Command::new("ld")
                            .arg("-o")
                            .arg(&output_file)
                            .arg("temp.o")
                            .status()
                            .expect("Failed to execute `ld` command");

                        if !status.success() {
                            eprintln!("Error: Linking for Linux failed");
                            exit(1);
                        }
                    }
                    "windows" => {
                        // Assemble for Windows
                        let nasm_args = vec!["-f", "win64", asm_file_path]; // Change to win32 format
                        let status = Command::new("nasm")
                            .args(nasm_args)
                            .status()
                            .expect("Failed to execute `nasm` command");

                        if !status.success() {
                            eprintln!("Error: Assembly for Windows failed");
                            exit(1);
                        }
                        let status = Command::new("link")
                            .arg(format!("/OUT:{}", output_file.display())) // Output filename
                            .arg("temp.obj") // The object file created by NASM
                            .status()
                            .expect("Failed to execute `link` command");
                        //let status = Command::new("/usr/bin/clang")
                        //     .arg("-o")
                        //     .arg(&output_file) // Output filename
                        //     .arg("temp.obj") // The object file created by NASM
                        //     .status()
                        //     .expect("Failed to execute `clang` command");

                        if !status.success() {
                            eprintln!("Error: Linking for Windows failed");
                            exit(1);
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

                println!("Successfully built for {}: {:?}", target, output_file);
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
