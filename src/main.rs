use std::{env::args, fs, path::Path, process::exit};
pub mod compilers;
pub mod utils;
use compilers::{compile::compile_linux, genasm::genasm_lin};
use utils::{fo::checkproj, token::gentoken};

fn main() {
    let projs: Vec<String> = args().collect();
    let mut index = 0;
    for i in projs {
        if index != 0 {
            let pc = checkproj(&i);
            match pc {
                Ok(_) => match fs::read_to_string(Path::new(format!("{}/main.nsc", i).as_str())) {
                    Ok(cnt) => {
                        let codes: Vec<&str> = cnt.split("\n").collect();
                        let tokens = gentoken(codes);
                        match tokens {
                            Ok(tokens) => {
                                println!("tokens : \n{:?}", tokens);
                                let asmc = genasm_lin(tokens);
                                //println!("ASM code:\n=> {}", asmc);

                                compile_linux(&asmc);
                            }
                            Err(e) => {
                                eprintln!("{}", e);
                                exit(0);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Error : Unable to read main file at provided project path |-> {}",
                            e
                        );
                        exit(0);
                    }
                },
                Err(e) => {
                    eprintln!("Error : {}", e);
                    exit(0);
                }
            }
        } else {
            index += 1;
        }
    }
}
