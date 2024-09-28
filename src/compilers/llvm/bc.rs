use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{exit, Command},
};

pub fn comp_c(c_code: &String, proj: &str, target: &str, project_name: &str) {
    println!("Target at BC (9): {}", target);
    let build_dir = Path::new(proj).join("build");
    if !build_dir.exists() {
        fs::create_dir_all(&build_dir).expect("Failed to create build directory");
    }

    let output_file = match target {
        "linux" => build_dir.join(project_name),
        "windows" => build_dir.join(format!("{}.exe", project_name)),
        "c" => build_dir.join(format!("{}.c", project_name)), // C file for 'c' target
        _ => {
            eprintln!("Error: Unsupported build target '{}'.", target);
            exit(1);
        }
    };

    // Handle 'c' target first: just write the C code to the file and exit
    if target == "c" {
        match File::create(&output_file) {
            Ok(mut c_file) => {
                if let Err(_) = c_file.write_all(c_code.as_bytes()) {
                    eprintln!(
                        "Error: Unable to write C code to file\nHint: Ensure correct permissions"
                    );
                    exit(1);
                }
                println!("C file generated at: {:?}", output_file);
            }
            Err(_) => {
                eprintln!("Error: Unable to create C file\nHint: Ensure correct permissions");
                exit(1);
            }
        }
        return; // Exit after generating the C file
    }

    // Create temporary C file path for linux/windows compilation
    let c_file_path = build_dir.join("temp.c");

    // Write the C code to the temporary C file
    match File::create(&c_file_path) {
        Ok(mut c_file) => {
            if let Err(_) = c_file.write_all(c_code.as_bytes()) {
                eprintln!("Error: Unable to write C code to temp file");
                exit(1);
            }
        }
        Err(_) => {
            eprintln!("Error: Unable to create temp C file");
            exit(1);
        }
    }

    // Compile the C code for other targets (linux/windows)
    let clang_args = vec![
        c_file_path.to_str().unwrap(),
        "-o",
        output_file.to_str().unwrap(),
        "-O3",    // Highest level of optimization
        "-static", // Static linking
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

    // Clean up the temporary C file
    fs::remove_file(c_file_path).expect("Failed to delete temporary C file");
}
