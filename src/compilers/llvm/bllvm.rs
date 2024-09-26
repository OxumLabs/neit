use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{exit, Command},
};

pub fn compile_llvm(llvm_ir: &String, proj: &str, target: &str, project_name: &str) {
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

    let llvm_file_path = "./temp.ll";
    match File::create(llvm_file_path) {
        Ok(mut llvmf) => match llvmf.write_all(llvm_ir.as_bytes()) {
            Ok(_) => {
                match target {
                    "linux" => {
                        let llc_args = vec!["-filetype=obj", llvm_file_path, "-o=temp.o"];
                        let status = Command::new("llc")
                            .args(llc_args)
                            .status()
                            .expect("Failed to execute `llc` command");

                        if !status.success() {
                            eprintln!("Error: Compilation to object file for Linux failed");
                        } else {
                            let status = Command::new("clang")
                                .arg("-o")
                                .arg(&output_file)
                                .arg("temp.o")
                                .arg("-nostdlib")
                                .arg("-Wl,--no-relax")
                                .status()
                                .expect("Failed to execute `clang` command");

                            if !status.success() {
                                eprintln!("Error: Linking for Linux failed");
                            } else {
                                println!(
                                    "Successfully built LLVM IR for {}: {:?}",
                                    target, output_file
                                );
                            }
                        }
                    }
                    "windows" => {
                        let llc_args = vec!["-filetype=obj", llvm_file_path, "-o=temp.obj"];
                        let status = Command::new("llc")
                            .args(llc_args)
                            .status()
                            .expect("Failed to execute `llc` command");

                        if !status.success() {
                            eprintln!("Error: Compilation to object file for Windows failed");
                        } else {
                            let status = Command::new("clang")
                                .arg("-o")
                                .arg(&output_file)
                                .arg("temp.obj")
                                .arg("-nostdlib")
                                .arg("-Wl,--no-relax")
                                .status()
                                .expect("Failed to execute `clang` command");

                            if !status.success() {
                                eprintln!("Error: Linking for Windows failed");
                            } else {
                                println!(
                                    "Successfully built LLVM IR for {}: {:?}",
                                    target, output_file
                                );
                            }
                        }
                    }
                    _ => {
                        eprintln!("Error: Unsupported target '{}'.", target);
                        exit(1);
                    }
                }

                fs::remove_file(llvm_file_path).expect("Failed to delete temporary LLVM file");
                if target == "linux" {
                    fs::remove_file("temp.o").expect("Failed to delete temporary object file");
                } else if target == "windows" {
                    fs::remove_file("temp.obj").expect("Failed to delete temporary object file");
                }
            }
            Err(_) => {
                eprintln!(
                    "Error: Unable to write LLVM code to file\nHint: Ensure correct permissions"
                );
                exit(1);
            }
        },
        Err(_) => {
            eprintln!("Error: Unable to create LLVM file\nHint: Ensure correct permissions");
            exit(1);
        }
    }
}
