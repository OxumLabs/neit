use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{exit, Command},
};

#[allow(unused_assignments)]
pub fn comp_c(c_code: &String, proj: &str, target: &str, project_name: &str) {
    let build_dir = Path::new(proj).join("build");
    if !build_dir.exists() {
        fs::create_dir_all(&build_dir).expect("Failed to create build directory");
    }

    let output_file = match target {
        "linux" => build_dir.join(project_name),
        "windows" => build_dir.join(format!("{}.exe", project_name)),
        "c" => build_dir.join(format!("{}.c", project_name)), // C file for 'c' target
        "llvm-ir" => build_dir.join(format!("{}.ll", project_name)),
        _ => {
            eprintln!("Error: Unsupported build target '{}'.", target);
            exit(1);
        }
    };

    // Handle 'c' target first: just write the C code to the file and exit
    if target == "c" {
        match File::create(&output_file) {
            Ok(mut c_file) => {
                let c_code = cfmt(&c_code);
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

    // Create temporary C file path for linux/windows/llvm-ir compilation
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

    // Determine the appropriate clang arguments
    let clang_args = if target == "llvm-ir" {
        // LLVM IR generation (no linking, just output the .ll file)
        vec![
            c_file_path.to_str().unwrap(),
            "-emit-llvm", // Generate LLVM IR
            "-S",         // Output as .ll file (text format)
            "-o",
            output_file.to_str().unwrap(),
        ]
    } else {
        // Compilation for Linux/Windows with optimizations
        vec![
            c_file_path.to_str().unwrap(),
            "-o",
            output_file.to_str().unwrap(),
            // Optimization Levels
            "-O3",
            "-march=native", // Use all available CPU features
            "-mtune=native", // Optimize for the host CPU
            // Static Linking
            "-static",      // Ensure fully static linking (no dynamic dependencies)
            "-fuse-ld=lld", // Use LLVM's faster linker
            // Link-Time Optimization (LTO)
            "-flto", // Enable link-time optimization (LTO) across all files
            // Function Optimizations
            "-finline-functions", // Aggressively inline functions to reduce function call overhead
            "-funroll-loops",     // Unroll loops to eliminate branching inside loops
            // Vectorization and SIMD Optimizations
            "-fvectorize",     // Automatically vectorize loops
            "-fslp-vectorize", // Apply vectorization to straight-line code
            "-mavx2",          // Use AVX2 instructions for vectorization (if supported by CPU)
            "-mfma",           // Use FMA (fused multiply-add) instructions for floating-point
            // Floating-Point Optimizations
            "-ffast-math", // Aggressive floating-point optimizations (may ignore strict IEEE compliance)
            "-ffinite-math-only", // Assume no NaNs or infinities
            "-fno-math-errno", // Don't set errno for math functions
            "-fassociative-math", // Allow reassociation of floating-point operations
            "-freciprocal-math", // Use reciprocal approximation for divisions
            // Memory and Cache Optimizations
            "-fstrict-aliasing", // Assume strict aliasing rules, which allows better optimizations
            "-fomit-frame-pointer", // Don't use a frame pointer (frees up a register)
            "-ffunction-sections", // Place each function in its own section for dead code elimination
            "-fdata-sections",     // Place data in its own sections for dead code elimination
            "-fmerge-all-constants", // Merge identical constants to reduce code size
            // Concurrency and Parallelism
            "-fopenmp", // Enable OpenMP support for parallelism
            // Debugging and Safety (Disable any runtime checks for release builds)
            "-DNDEBUG",                 // Disable assertions
            "-fstack-protector-strong", // Stack protection for security, but still lightweight
            // Linker Final Static Flags
            "-pthread", // Link with pthread for multi-threading (needed for static binaries on Linux)
        ]
    };

    // Execute the clang command
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

/* ------------------------------------------- */
/* FORMAT C CODE */
/* ------------------------------------------- */
pub fn cfmt(code: &str) -> String {
    let mut formatted_code = String::new();
    let mut indent_level = 0;
    let mut in_multiline_comment = false;
    let lines = code.lines();

    for line in lines {
        let trimmed_line = line.trim();
        if in_multiline_comment {
            formatted_code.push_str(&format!("{}\n", line));
            if trimmed_line.ends_with("*/") {
                in_multiline_comment = false;
            }
            continue;
        }
        if trimmed_line.starts_with("/*") {
            in_multiline_comment = true; // Start of multiline comment
            formatted_code.push_str(&format!("{}\n", line));
            continue;
        }

        // Manage indentation based on braces
        if trimmed_line.ends_with('{') {
            formatted_code.push_str(&format!(
                "{}{}\n",
                "    ".repeat(indent_level),
                trimmed_line
            ));
            indent_level += 1;
            continue;
        } else if trimmed_line == "}" {
            indent_level -= 1;
            formatted_code.push_str(&format!(
                "{}{}\n",
                "    ".repeat(indent_level),
                trimmed_line
            ));
            continue;
        }
        formatted_code.push_str(&format!(
            "{}{}\n",
            "    ".repeat(indent_level),
            trimmed_line
        ));
    }
    formatted_code
}
