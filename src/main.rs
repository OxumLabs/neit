use std::{
    env,
    fs::{self, read_to_string},
    path::Path,
    process::exit,
};

pub mod compilers;
pub mod utils;
use compilers::{compile::compile, genasm_lin::genasm_lin, genasm_win::genasm_win};
use utils::{fo::checkproj, token::gentoken};

fn main() {
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
        // Use the current directory if no project path is provided
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

    // Validate the project path

    match cmd.trim() {
        "build" => build_project(proj),
        "run" => run_project(proj),
        "new" => create_new_project(proj),
        _ => {
            eprintln!(
                "Error: Invalid command '{}'.\nSupported commands:\n - build\n - run\n - new",
                cmd
            );
            exit(1);
        }
    }
}
#[allow(unused)]
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
                let asm_code = match target.as_str() {
                    "linux" => genasm_lin(&tokens),
                    "windows" => genasm_win(&tokens),
                    _ => {
                        eprintln!("Error: Unsupported build target '{}'. Supported targets: linux, windows", target);
                        continue; // Skip to the next target
                    }
                };

                // Compile the generated assembly code, passing the project name
                compile(&asm_code, proj, &target, &project_name);
            }
        }
        Err(e) => {
            eprintln!("Error processing tokens: {}", e);
            exit(1);
        }
    }
}
fn run_project(proj: &str) {
    println!("Running project at: {}", proj);
    // Implement run functionality here
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
    let main_file_content = r#"_WRT("Hello, world")"#;
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
