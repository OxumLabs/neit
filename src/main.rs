use std::{
    env::{self, consts::OS},
    fs::{self, read_to_string, File},
    io::Write,
    path::Path,
    process::{exit, Command, Stdio},
};

pub mod compilers;
pub mod utils;
use compilers::{
    compile::check_tools_installed,
    llvm::{bc::comp_c, c::to_c},
};
use utils::{fo::checkproj, token::gentoken};

fn main() {
    let cti = check_tools_installed();
    match cti {
        Ok(_) => {}
        Err(e) => {
            println!("Error: One or more tools not installed\n-> {}", e);
            exit(1);
        }
    }
    let args: Vec<String> = env::args().collect();

    // Ensure we have the required command and project path
    if args.len() < 2 {
        eprintln!(
            "Error: Missing command.\nUsage: {} <command> [<project_path>]",
            args[0]
        );
        exit(1);
    }

    // Determine the project path
    let proj = if args.len() > 2 {
        &args[2]
    } else {
        &match env::current_dir() {
            Ok(path) => path.to_string_lossy().into_owned(),
            Err(_) => {
                eprintln!("Error: Unable to determine current directory.");
                exit(1);
            }
        }
    };

    // Validate the command argument
    let cmd = &args[1];

    match cmd.trim() {
        "build" => build_project(proj),
        "run" => run_project(proj),
        "new" => create_new_project(proj),
        "help" => display_help(),
        "target-list" => display_target_list(),
        _ => {
            eprintln!(
                "Error: Invalid command '{}'.\nSupported commands:\n - help\n - target-list\n - build\n - run\n - new",
                cmd
            );
            exit(1);
        }
    }
}

fn build_project(proj: &str) {
    println!("Building the project at: {}", proj);

    // Check if the project is valid
    if let Err(e) = checkproj(&proj.to_string()) {
        eprintln!("{}", e);
        exit(1);
    }

    // Define file paths for the main.nsc and project.conf files
    let main_file_path = format!("{}/main.nsc", proj);
    let config_file_path = format!("{}/project.conf", proj);

    // Read the project configuration from 'project.conf'
    let config_content = match read_to_string(&config_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "Error: Unable to read 'project.conf' file at '{}': {}",
                proj, e
            );
            exit(1);
        }
    };

    // Parse the project configuration to get the name and build targets
    let mut project_name = String::new();
    let mut build_targets = Vec::new();

    for line in config_content.lines() {
        if line.starts_with("Name:") {
            project_name = line["Name:".len()..].trim().to_string();
        } else if line.starts_with("Build:") {
            let targets = line["Build:".len()..].trim();
            build_targets = targets
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .collect();
        }
    }

    // Ensure the project name is found
    if project_name.is_empty() {
        eprintln!("Error: No project name found in 'project.conf'.");
        exit(1);
    }

    // Read the main.nsc file
    let main_content = match read_to_string(&main_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: Unable to read 'main.nsc' file at '{}': {}", proj, e);
            exit(1);
        }
    };

    let code: Vec<&str> = main_content.lines().collect();
    match gentoken(code) {
        Ok(tokens) => {
            // Process each build target
            for target in build_targets {
                // Generate assembly code based on the target
                let asm_code = to_c(&tokens);

                // Compile the generated assembly code, passing the project name
                comp_c(&asm_code, proj, &target, &project_name);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

fn run_project(proj: &str) {
    println!("Running project at: {}", proj);

    // Check if the project is valid
    if let Err(e) = checkproj(&proj.to_string()) {
        eprintln!("{}", e);
        exit(1);
    }

    // Read the main.nsc file
    let mf = format!("{}/main.nsc", proj);
    let mc = match read_to_string(&mf) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: Unable to read 'main.nsc' file at '{}': {}", proj, e);
            exit(1);
        }
    };

    let cds: Vec<&str> = mc.split("\n").collect();
    match gentoken(cds) {
        Ok(tkns) => {
            let dtf = format!("{}/_.c", proj); // Temporary C file
            let outf = match OS {
                "windows" => format!("{}/_.exe", proj),
                "linux" => format!("{}/_.out", proj),
                _ => {
                    eprintln!("Error: Unknown OS");
                    exit(1);
                }
            };

            // Create the C file
            match File::create(&dtf) {
                Ok(mut dtcf) => {
                    let c = to_c(&tkns);

                    // Write C code to the temporary C file
                    match dtcf.write_all(c.as_bytes()) {
                        Ok(_) => {
                            // Compile the C file into an executable
                            let cargs = vec![
                                dtf.clone().to_string(),
                                "-o".to_string(),
                                outf.clone(),
                                "-fuse-ld=lld".to_string(),
                            ];
                            let cmd = Command::new("clang").args(cargs).status();

                            match cmd {
                                Ok(ok) => {
                                    if ok.success() {
                                        let status = Command::new(outf.clone())
                                            .stdout(Stdio::inherit()) // Inherit stdout for real-time output
                                            .stderr(Stdio::inherit()) // Inherit stderr for real-time error reporting
                                            .status()
                                            .expect("Failed to run the executable");

                                        // Wait for the child process to finish
                                        if !status.success() {
                                            eprintln!("Process finished with an error.");
                                        }

                                        // Clean up temporary files after execution
                                        if let Err(e) = fs::remove_file(&dtf) {
                                            eprintln!(
                                                "Warning: Could not delete temp C file '{}': {}",
                                                dtf, e
                                            );
                                        }
                                        if let Err(e) = fs::remove_file(&outf) {
                                            eprintln!(
                                                "Warning: Could not delete temp executable '{}': {}",
                                                outf, e
                                            );
                                        }
                                    } else {
                                        eprintln!("Error: Compilation failed.");
                                    }
                                }
                                Err(_) => {
                                    eprintln!("Error: Failed to run clang or lld!");
                                    exit(1);
                                }
                            }
                        }
                        Err(_) => {
                            eprintln!("Error: Unable to write to C file.");
                            exit(1);
                        }
                    }
                }
                Err(_) => {
                    eprintln!("Error: Unable to create C file.");
                    exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

pub fn create_new_project(proj: &str) {
    println!("Creating a new project at: {}", proj);
    let proj_path = Path::new(proj);

    // Check if the project directory already exists
    if proj_path.exists() {
        eprintln!("Error: Project directory '{}' already exists.", proj);
        exit(1);
    }

    // Create the project directory if it does not exist
    if let Err(e) = fs::create_dir_all(proj_path) {
        eprintln!(
            "Error: Failed to create project directory '{}': {}",
            proj, e
        );
        exit(1);
    }

    // Create main.nsc file
    let main_file_content = "println(\"Hello, world\")\n#The Neit Programming Language";
    let main_file_path = proj_path.join("main.nsc");
    if let Err(e) = fs::write(&main_file_path, main_file_content) {
        eprintln!("Error: Failed to create 'main.nsc' file: {}", e);
        exit(1);
    }

    // Create project.conf file
    let project_name = proj_path
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("unknown_project"))
        .to_string_lossy();
    let os = std::env::consts::OS;

    let config_content = format!("Name: {}\nAuthor: USER\nBuild: {}\n", project_name, os);
    let config_file_path = proj_path.join("project.conf");
    if let Err(e) = fs::write(&config_file_path, config_content) {
        eprintln!("Error: Failed to create 'project.conf' file: {}", e);
        exit(1);
    }

    println!("Project created successfully at: {}", proj);
}

fn display_help() {
    println!("Available commands:");
    println!(" - build       : Builds the project (if in the project dir no need to specify project path).");
    println!(" - run         : Runs the project (if in the project dir no need to specify project path).");
    println!(" - new         : Creates a new project in a new folder in currewnt dir named by the given project name.");
    println!(" - help        : Displays this help message.");
    println!(" - target-list : Displays available build targets and their purposes.");
    exit(0);
}

fn display_target_list() {
    println!("Available targets:");
    println!(" - llvm-ir     : Generates LLVM intermediate representation.");
    println!(" - c           : Generates C code.");
    println!(" - windows     : Compiles for Windows operating system.");
    println!(" - linux       : Compiles for Linux operating system.");
    exit(0);
}
