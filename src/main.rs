use std::{env::args, fs::File, io::Read, path::Path, process::exit};

use build_system::linux_b::linux_b_64;
use c_gens::makec::make_c;
use parse_systems::parse;
use tok_system::{lexer::LexicalAnalysis, tokens::Token};
pub mod tok_system;
pub mod parse_systems;
pub mod c_gens;
pub mod build_system;
pub mod nulibc;
pub mod err_system;

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() <= 2 {
        //add better errs
        eprintln!("Invalid args!");
        exit(1);
    }
    //debug code
    let cmd = &args[1];
    let path = &args[2];
    if cmd == "build" {
        //check if path exists
        let proj = Path::new(path);
        if !proj.exists() {
            eprintln!("unable to find {}", path);
            exit(1);
        }
        if proj.is_dir() {
            eprintln!("currently not compiling dirs");
            exit(1);
        }
        if let Ok(mut projf) = File::open(path) {
            let mut code = String::new();
            if let Err(e) = projf.read_to_string(&mut code) {
                eprintln!("error reading file : {}", e);
            }
            //tokenize the code!
            let mut tokens: Vec<Token> = Vec::new();
            tokens.run_lexical_analysis(&code);
            let ast = parse(&tokens, &code);
            let c_code = make_c(&ast, true);
            match linux_b_64(&c_code){
                Ok(()) => println!("build success"),
                Err(e) => eprintln!("build failed : {}",e),
            }
        } else {
            eprintln!("unable to open file {}", path);
        }
    }
}
