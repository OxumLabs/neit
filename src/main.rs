use std::{
    env::{self, consts::OS},
    fs::{self, read_to_string, File},
    io::Write,
    path::Path,
    process::{exit, Command, Stdio},
};

pub mod compilers;
pub mod ntune;
pub mod utils;
use compilers::{
    compile::{check_tools_installed, compile},
    genasm_lin::genasm_lin,
    genasm_win::genasm_win,
    llvm::{bc::comp_c, c::to_c},
};
use ntune::ntune::{gen_grm, process_grammar_file, process_neit_file, Grammar};
use utils::{fo::checkproj, token::gentoken};

fn main() {
    let cti = check_tools_installed();

    match cti {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "✘ Oops! It seems like one or more tools are missing from the toolbox!\n\
                ➔ Let’s get those tools installed so we can get back to work!\n\
                ERROR: {}",
                e
            );
            exit(1);
        }
    }
    let args: Vec<String> = env::args().collect();

    // Ensure we have the required command and project path
    if args.len() < 2 {
        eprintln!(
            "✘ Oops! It looks like you forgot to include a command!\n\
            Usage: {} <command> [<project_path>]\n\
            ➔ Let’s get that command in there so we can keep the fun going!",
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
                eprintln!(
                    "✘ Oops! I can't seem to find the current directory—it's like it vanished!\n\
                    ➔ Let’s check if it’s hiding somewhere or if we need to give it a little nudge!"
                );
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
                "✘ Oops! It looks like the command '{}' is not valid—it's like trying to use a banana as a phone!\n\
                ➔ Supported commands:\n\
                - help: Need a hand?\n\
                - target-list: What’s on the menu?\n\
                - build: Let’s construct something awesome!\n\
                - run: Time to get moving!\n\
                - new: Ready for a fresh start?\n\
                ➔ Let’s stick to these commands and keep the fun rolling!",
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
                "✘ Uh-oh! I tried to read the 'project.conf' file at '{}' but it seems to be missing!\n\
                ➔ Error: {}\n\
                Let’s find out what’s going on—maybe it’s just hiding?",
                proj, e
            );

            exit(1);
        }
    };

    // Parse the project configuration to get the name, build targets, and grammar
    let mut project_name = String::new();
    let mut build_targets = Vec::new();
    let mut input_grammar_file = String::new();
    let mut use_grammar_file = String::new();

    for line in config_content.lines() {
        if line.starts_with("Name:") {
            project_name = line["Name:".len()..].trim().to_string();
        } else if line.starts_with("Build:") {
            let targets = line["Build:".len()..].trim();
            build_targets = targets
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .collect();
        } else if line.starts_with("input_grammar=") {
            input_grammar_file = line["input_grammar=".len()..].trim().to_string();
        } else if line.starts_with("use_grammar=") {
            use_grammar_file = line["use_grammar=".len()..].trim().to_string();
        }
    }

    // Ensure the project name is found
    if project_name.is_empty() {
        eprintln!(
            "✘ Uh-oh! It seems like I couldn’t find a project name in 'project.conf'—it’s like looking for a needle in a haystack!\n\
            ➔ Let’s make sure you’ve got a name in there so we can get this party started!"
        );
        exit(1);
    }

    // Read the main.nsc file
    let main_content = match read_to_string(&main_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "✘ Uh-oh! I tried to read the 'main.nsc' file at '{}' but it seems to be missing!\n\
                ➔ Error: {}\n\
                Let’s track it down and see what’s going on—maybe it needs a map!",
                proj, e
            );
            exit(1);
        }
    };

    // Handle grammar processing
    let mut updated_content = main_content;

    if !input_grammar_file.is_empty() {
        let mut usrgrm: Vec<Grammar> = Vec::new();
        let defgen = gen_grm();
        process_grammar_file(&format!("{}/{}", proj, input_grammar_file), &mut usrgrm);
        updated_content = process_neit_file(&main_file_path, &usrgrm, &defgen);
    }

    let code: Vec<&str> = updated_content.lines().collect();
    match gentoken(code, Vec::new(), false) {
        Ok(tokens) => {
            // Process each build target
            for target in build_targets {
                // Generate assembly code based on the target
                let mut asm_code = String::new();
                if target != "win_asm" && target != "lin_asm" {
                    asm_code = to_c(&tokens); // Handle unsupported targets
                } else {
                    if target == "win_asm" {
                        asm_code = genasm_win(&tokens); // Generate Windows ASM
                        println!("\n\nWindows ASM :\n{}\n\n", asm_code);
                    } else {
                        asm_code = genasm_lin(&tokens); // Generate Linux ASM
                        println!("\n\nLinux ASM :\n{}\n\n", asm_code);
                    }
                }

                // Compile the generated assembly code, passing the project name
                if target == "win_asm" {
                    compile(&asm_code, proj, &target, &project_name);
                } else if target == "lin_asm" {
                    compile(&asm_code, proj, &target, &project_name);
                } else {
                    comp_c(&asm_code, proj, &target, &project_name);
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

#[allow(unused)]
fn run_project(proj: &str) {
    println!("Running project at: {}", proj);

    // Check if the project is valid
    if let Err(e) = checkproj(&proj.to_string()) {
        eprintln!("{}", e);
        exit(1);
    }

    // Read the main.nsc file
    let mf = format!("{}/main.nsc", proj);
    let mut mc = match read_to_string(&mf) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "✘ Uh-oh! I tried to read the 'main.nsc' file at '{}' but it seems to be missing!\n\
                ➔ Error: {}\n\
                Let’s track it down and see what’s going on—maybe it needs a map!",
                proj, e
            );
            exit(1);
        }
    };
    let cf = format!("{}/project.conf", proj);
    let cc = match read_to_string(&cf) {
        Ok(cc) => cc,
        Err(_) => {
            eprintln!("✘ Uh-oh! I tried to find 'project.conf' file at '{}' but I cant find it!\n➔ Make sure it is there and didn't run away from me?",cf);
            exit(1)
        }
    };
    let mut icm = false;
    let mut igf = String::new();
    let mut ugf = String::new();
    for cfc in cc.lines() {
        if cfc.trim().starts_with("[sec-start] grammar") {
            icm = true;
        } else if cfc.trim().starts_with("[sec-end] grammar") {
            icm = false;
        } else {
            if icm {
                if cfc.trim().starts_with("input_grammar=") {
                    //println!("igf : {}", cfc);
                    igf = cfc.trim_start_matches("input_grammar=").to_string();
                    //println!("igf : {}", igf);
                } else if cfc.trim().starts_with("use_grammar=") {
                    ugf = cfc.trim_start_matches("use_grammar=").to_string();
                }
            }
        }
    }
    if !igf.is_empty() {
        //println!("main content :\n {}", mc);
        let mut usrgrm: Vec<Grammar> = Vec::new();
        let defgen = gen_grm();
        process_grammar_file(&format!("{}/{}", proj, igf), &mut usrgrm);
        let nmc = process_neit_file(&mf, &usrgrm, &defgen);
        //println!("nmc : {}", nmc);
        mc = nmc;
        //println!("main content after parsing :\n {}", mc);
    }

    let cds: Vec<&str> = mc.split("\n").collect();
    match gentoken(cds, Vec::new(), false) {
        Ok(tkns) => {
            let dtf = format!("{}/_.c", proj); // Temporary C file
            let outf = match OS {
                "windows" => format!("{}/_.exe", proj),
                "linux" => format!("{}/_.out", proj),
                _ => {
                    eprintln!(
                        "✘ Oops! I can't seem to figure out what operating system we're on—it's like trying to find a unicorn in a haystack!\n\
                        ➔ If you could use Windows, macOS, or Linux, that would be super helpful! Let’s get this sorted out!"
                    );
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
                                "-Wno-format".to_string(),
                            ];
                            let cmd = Command::new("clang").args(cargs).status();

                            match cmd {
                                Ok(ok) => {
                                    if ok.success() {
                                        let status = Command::new(outf.clone())
                                            .stdout(Stdio::inherit())
                                            .stderr(Stdio::inherit())
                                            .status()
                                            .unwrap();
                                        if !status.success() {
                                            eprintln!(
                                                "✘ Oh no! It seems that running the program failed.\n\
                                                ➔ Let’s check the code and make sure everything is in order!"
                                            );
                                            exit(1);
                                        }
                                    } else {
                                        eprintln!(
                                            "✘ Oh no! It seems that the compilation failed.\n\
                                            ➔ Let’s check for any errors in the code and try again!"
                                        );
                                        exit(1);
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "✘ Uh-oh! I tried to run the command to compile but it didn’t go through.\n\
                                        ➔ Error: {}\n\
                                        Let’s see if we can troubleshoot this together!",
                                        e
                                    );
                                    exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "✘ Uh-oh! I tried to write to the temporary C file but it seems to be having issues!\n\
                                ➔ Error: {}\n\
                                Let’s check if there’s enough space or if something else is in the way!",
                                e
                            );
                            exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "✘ Oops! I tried to create the temporary C file but it seems to be stuck!\n\
                        ➔ Error: {}\n\
                        Let’s see if we can free it up and carry on!",
                        e
                    );
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

fn create_new_project(proj: &str) {
    println!("Creating a new project at: {}", proj);

    // Check if the project path already exists
    if Path::new(proj).exists() {
        eprintln!(
            "✘ Uh-oh! It seems like a project already exists at '{}'.\n\
            ➔ Let’s choose a different location to start fresh!",
            proj
        );
        exit(1);
    }

    // Create the project directory
    match fs::create_dir_all(proj) {
        Ok(_) => {
            let main_file = format!("{}/main.nsc", proj);
            let config_file = format!("{}/project.conf", proj);

            // Create the main.nsc file with a basic template
            match File::create(&main_file) {
                Ok(mut file) => {
                    let template = "// This is the main.nsc file for your project\n\n";
                    match file.write_all(template.as_bytes()) {
                        Ok(_) => {
                            println!("Created: {}", main_file);
                        }
                        Err(e) => {
                            eprintln!(
                                "✘ Oops! I tried to write to the main.nsc file but it seems to be having issues!\n\
                                ➔ Error: {}\n\
                                Let’s check if there’s enough space or if something else is in the way!",
                                e
                            );
                            exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "✘ Uh-oh! I tried to create the main.nsc file but it seems to be stuck!\n\
                        ➔ Error: {}\n\
                        Let’s see if we can free it up and carry on!",
                        e
                    );
                    exit(1);
                }
            }

            // Create the project.conf file with a basic template
            match File::create(&config_file) {
                Ok(mut file) => {
                    let template = "Name: MyProject\nBuild: lin_asm, win_asm\n";
                    match file.write_all(template.as_bytes()) {
                        Ok(_) => {
                            println!("Created: {}", config_file);
                        }
                        Err(e) => {
                            eprintln!(
                                "✘ Oops! I tried to write to the project.conf file but it seems to be having issues!\n\
                                ➔ Error: {}\n\
                                Let’s check if there’s enough space or if something else is in the way!",
                                e
                            );
                            exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "✘ Uh-oh! I tried to create the project.conf file but it seems to be stuck!\n\
                        ➔ Error: {}\n\
                        Let’s see if we can free it up and carry on!",
                        e
                    );
                    exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!(
                "✘ Oops! I tried to create the project directory but it seems to be stuck!\n\
                ➔ Error: {}\n\
                Let’s see if we can free it up and carry on!",
                e
            );
            exit(1);
        }
    }
}

fn display_help() {
    println!("Help: Available Commands:");
    println!("- build: Am gonna cook the meal and package it for you!");
    println!("- run: I will get this project to run as you told me to");
    println!("- new: I will serve you with a fresh new plate of (the name you gonna choose)");
    println!("- help: C'mon you already ran it..duh!?");
    println!("- target-list: List available build targets");
}

fn display_target_list() {
    println!("Available Build Targets:");
    println!("- win_asm: I will make windows binary compiled from Windows ASM code for x86_64 ");
    println!("- lin_asm: I will make Linux binary compiled from a  pure Linux ASM for x86_64");
    println!("- windows: I will make windows binary compiled using clang");
    println!("- lin_asm: I will make Linux binary compiled using clang");
    println!("- llvm-ir : I will put out my llvm-ir code just for you :3 ");
    println!("- c : I will put out my C code just for you :3");
}
