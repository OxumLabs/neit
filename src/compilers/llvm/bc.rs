use std::{
    env::consts::OS,
    fs::{self, File},
    io::Write,
    path::Path,
    process::{exit, Command},
};
#[allow(clippy::redundant_pattern_matching)]
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
                let c_code = cfmt(c_code);
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
        if OS == "linux" {
            vec![
                c_file_path.to_str().unwrap(),
                "-o",
                output_file.to_str().unwrap(),
                "-O3",                      // Optimize for maximum speed
                "-march=native",            // Generate code optimized for the host CPU
                "-mtune=native",            // Tune code for the host CPU
                "-static",                  // Ensure fully static linking
                "-fuse-ld=lld",             // Use LLVM's faster linker
                "-flto=thin",               // Use Thin LTO for faster link-time optimizations
                "-finline-functions",       // Aggressively inline functions
                "-funroll-loops",           // Unroll loops
                "-fvectorize",              // Automatically vectorize loops
                "-fslp-vectorize",          // Apply vectorization to straight-line code
                "-mavx2",                   // Use AVX2 instructions (if supported by CPU)
                "-mfma",                    // Use FMA instructions for floating-point operations
                "-ffast-math",              // Enable aggressive floating-point optimizations
                "-ffinite-math-only",       // Assume no NaNs or infinities
                "-fno-math-errno",          // Don't set errno for math functions
                "-fassociative-math",       // Allow reassociation of floating-point operations
                "-freciprocal-math",        // Use reciprocal approximation for divisions
                "-fstrict-aliasing",        // Assume strict aliasing rules
                "-fomit-frame-pointer",     // Omit frame pointer for more registers
                "-ffunction-sections",      // Place functions in separate sections
                "-fdata-sections",          // Place data in separate sections
                "-fmerge-all-constants",    // Merge identical constants
                "-DNDEBUG",                 // Disable assertions
                "-fstack-protector-strong", // Enable stack protection
                "-pthread",                 // Link with pthread for multi-threading
                "-pipe", // Use pipes rather than temporary files for communication between processes (speeds up compilation)
                "-flto-jobs=4", // Automatically parallelize LTO across available CPUs
                "-Wl,--threads=4", // Set the number of threads for the linker based on CPU cores
                "-Wl,--gc-sections", // Garbage collect unused sections for smaller binaries
            ]
        } else {
            vec![
                c_file_path.to_str().unwrap(),
                "-o",
                output_file.to_str().unwrap(),
                "-O3",                      // Optimize for maximum speed
                "-march=native",            // Generate code optimized for the host CPU
                "-mtune=native",            // Tune code for the host CPU
                "-static",                  // Ensure fully static linking
                "-fuse-ld=lld",             // Use LLVM's faster linker
                "-flto=thin",               // Use Thin LTO for faster link-time optimizations
                "-finline-functions",       // Aggressively inline functions
                "-funroll-loops",           // Unroll loops
                "-fvectorize",              // Automatically vectorize loops
                "-fslp-vectorize",          // Apply vectorization to straight-line code
                "-mavx2",                   // Use AVX2 instructions (if supported by CPU)
                "-mfma",                    // Use FMA instructions for floating-point operations
                "-ffast-math",              // Enable aggressive floating-point optimizations
                "-ffinite-math-only",       // Assume no NaNs or infinities
                "-fno-math-errno",          // Don't set errno for math functions
                "-fassociative-math",       // Allow reassociation of floating-point operations
                "-freciprocal-math",        // Use reciprocal approximation for divisions
                "-fstrict-aliasing",        // Assume strict aliasing rules
                "-fomit-frame-pointer",     // Omit frame pointer for more registers
                "-ffunction-sections",      // Place functions in separate sections
                "-fdata-sections",          // Place data in separate sections
                "-fmerge-all-constants",    // Merge identical constants
                "-fopenmp",                 // Enable OpenMP support for parallelism
                "-DNDEBUG",                 // Disable assertions
                "-fstack-protector-strong", // Enable stack protection
                "-pthread",                 // Link with pthread for multi-threading
                "-pipe", // Use pipes for faster communication between compilation stages
                "-flto-jobs=auto", // Automatically parallelize LTO using all available CPUs
                "-Wl,/OPT:REF", // Linker optimization: eliminate unreferenced code/data
                "-Wl,/OPT:ICF", // Identical COMDAT folding to reduce binary size
            ]
        }
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
