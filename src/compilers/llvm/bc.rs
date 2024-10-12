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
            eprintln!(
                "✘ Whoops! Target '{}' doesn't seem like a thing I can handle.",
                target
            );
            eprintln!("→ Hint: Maybe try 'linux', 'windows', 'c', or 'llvm-ir'?");
            eprintln!("⚙ [Oops occurred in: comp_c at target matching]");
            exit(1);
        }
    };

    // Collect messages for printing later
    let mut messages = Vec::new();

    // Handle 'c' target first: just write the C code to the file and exit
    if target == "c" {
        match File::create(&output_file) {
            Ok(mut c_file) => {
                let c_code = cfmt(c_code);
                if let Err(_) = c_file.write_all(c_code.as_bytes()) {
                    messages.push("✘ Eeeek! I tried to write your C code but... it slipped through my fingers.".to_string());
                    messages.push(
                        "→ Hint: Double-check those file permissions before I try again!"
                            .to_string(),
                    );
                    messages
                        .push("⚙ [Location: comp_c while writing C code to the file]".to_string());
                    print_messages(&messages);
                    exit(1);
                }
                messages.push(format!(
                    "ℹ Boom! Your Neit has pukeed out C file which is ready at: {:?}",
                    output_file
                ));
            }
            Err(_) => {
                messages.push("✘ Uh-oh, I'm blocked! Can't create the C file. File permissions are pesky little things, huh?".to_string());
                messages.push("→ Hint: File permissions, check 'em out!".to_string());
                messages.push("⚙ [Location: comp_c creating C file]".to_string());
                print_messages(&messages);
                exit(1);
            }
        }
        print_messages(&messages);
        return; // Exit after generating the C file
    }

    // Create temporary C file path for linux/windows/llvm-ir compilation
    let c_file_path = build_dir.join("temp.c");

    // Write the C code to the temporary C file
    match File::create(&c_file_path) {
        Ok(mut c_file) => {
            if let Err(_) = c_file.write_all(c_code.as_bytes()) {
                messages.push(
                    "✘ Uh-oh! I tried to scribble your C code, but something's not right."
                        .to_string(),
                );
                messages.push(
                    "→ Hint: Check if the temp file is allowed to be written on.".to_string(),
                );
                messages.push("⚙ [Location: comp_c writing to temp file]".to_string());
                print_messages(&messages);
                exit(1);
            }
        }
        Err(_) => {
            messages.push(
                "✘ Aha! Caught in a trap—can't even create the temporary C file.".to_string(),
            );
            messages.push("→ Hint: Permissions? Disk space?".to_string());
            messages.push("⚙ [Location: comp_c creating temp C file]".to_string());
            print_messages(&messages);
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
                "-Wno-format",
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
                "-pipe",                    // Use pipes for faster communication
                "-flto-jobs=4",             // Parallelize LTO across CPUs
                "-Wl,--threads=4",          // Set linker threads to 4
                "-Wl,--gc-sections",        // Garbage collect unused sections
            ]
        } else {
            vec![
                c_file_path.to_str().unwrap(),
                "-o",
                output_file.to_str().unwrap(),
                "-O3", // Optimize for maximum speed
                "-Wno-format",
                "-march=native",         // Generate code optimized for the host CPU
                "-mtune=native",         // Tune code for the host CPU
                "-static",               // Fully static linking
                "-fuse-ld=lld",          // Use LLVM's linker
                "-flto=thin",            // Thin LTO optimizations
                "-finline-functions",    // Inline functions aggressively
                "-funroll-loops",        // Unroll loops
                "-fvectorize",           // Automatically vectorize loops
                "-fslp-vectorize",       // Straight-line code vectorization
                "-mavx2",                // Use AVX2 instructions
                "-mfma",                 // Use FMA instructions
                "-ffast-math",           // Aggressive floating-point optimizations
                "-ffinite-math-only",    // No NaNs or infinities
                "-fno-math-errno",       // Don't set errno for math functions
                "-fassociative-math",    // Allow reassociation of floating-point ops
                "-freciprocal-math",     // Approximate reciprocals for divisions
                "-fstrict-aliasing",     // Assume strict aliasing rules
                "-fomit-frame-pointer",  // Omit frame pointer for extra registers
                "-ffunction-sections",   // Separate functions into sections
                "-fdata-sections",       // Separate data into sections
                "-fmerge-all-constants", // Merge constants
                "-DNDEBUG",              // Disable assertions
                "-fstack-protector-strong", // Enable stack protection
                "-pthread",              // Multi-threading support
                "-pipe",                 // Use pipes
                "-flto-jobs=auto",       // Parallelize LTO
                "-Wl,/OPT:REF",          // Linker optimization
                "-Wl,/OPT:ICF",          // COMDAT folding
            ]
        }
    };

    // Attempt to execute clang command
    let clang_status = Command::new("clang").args(&clang_args).status();

    // Check if clang was successful
    match clang_status {
        Ok(status) if status.success() => {
            messages.push(format!(
                "ℹ Success! Neit code compiled for target '{}'. Output at: {:?}",
                target, output_file
            ));
        }
        Ok(_) => {
            messages.push("✘ Oops! Clang failed to compile your code...".to_string());
            messages.push("→ Hint: Check for errors above this message!".to_string());
            messages.push("⚙ [Location: comp_c while running clang]".to_string());
            print_messages(&messages);
            exit(1);
        }
        Err(e) => {
            messages.push("✘ Whoops! Clang went missing...".to_string());
            messages.push("→ Hint: Make sure it's installed and in your PATH!".to_string());
            messages.push(format!("⚙ [Error: {:?}]", e));
            print_messages(&messages);
            exit(1);
        }
    }

    print_messages(&messages);
}
fn print_messages(messages: &[String]) {
    for msg in messages {
        println!("{}", msg);
    }
}

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
