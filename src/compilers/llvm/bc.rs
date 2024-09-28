use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{exit, Command},
};

pub fn comp_c(c_code: &String, proj: &str, target: &str, project_name: &str) {
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

    // Create temporary C file
    let c_file_path = "./temp.c";
    match File::create(c_file_path) {
        Ok(mut c_file) => {
            // Write the C code to the temporary file
            if let Err(_) = c_file.write_all(c_code.as_bytes()) {
                eprintln!(
                    "Error: Unable to write C code to file\nHint: Ensure correct permissions"
                );
                exit(1);
            }
        }
        Err(_) => {
            eprintln!("Error: Unable to create C file\nHint: Ensure correct permissions");
            exit(1);
        }
    }

    // Compile the C code with optimizations and static linking
    let clang_args = vec![
        c_file_path,
        "-o",
        output_file.to_str().unwrap(),
        "-O3",            // Highest level of optimization
        "-static",        // Static linking
        "-lm",            // Link math library if needed
        "-Wl,--no-relax", // Additional linker options
    ];

    let status = Command::new("clang")
        .args(clang_args)
        .status()
        .expect("Failed to execute `clang` command");

    // Check if compilation succeeded
    if !status.success() {
        eprintln!("Error: Compilation for {} failed", target);
        exit(1);
    } else {
        println!(
            "Successfully built C code for {}: {:?}",
            target, output_file
        );
    }

    // Clean up temporary files
    fs::remove_file(c_file_path).expect("Failed to delete temporary C file");
}
