/*
cls -> clear screen
> -> print
@ -> func
    - [fun_body]
.. -> input
. -> var
- Bytecode generation without new lines
- Clear separators used for readability and compactness
*/
///
/// 
/// NOTE: This code is completely unused and not needed though for some reason is still there in the project
/// 
/// 
/// 
use std::{thread::sleep, time::Duration};

use colored::Colorize;

use crate::p::{VVal, NST};

pub fn nrunp(nst: &Vec<NST>, target: &str) -> String {
    if target == "linux" || target == "windows" {
        let bc = genc(nst, target);
        println!("{}", bc);

        str2bin(&bc)
    } else {
        String::new()
    }
}
#[allow(unused)]
pub fn genc(nst: &Vec<NST>, target: &str) -> String {
    let mut bc = String::with_capacity(1024);

    for nst in nst {
        match nst {
            NST::NCLRSCRN => {
                bc.push_str("cls;"); // Clear screen command
            }
            NST::PRINT(txt) => {
                bc.push_str(&format!(">{};", txt)); // Print command with separator
            }
            NST::Func(name, args, nsts) => {
                let body = genc(nsts, target);
                let body_cleaned = body.replace("\n", ";");
                //bc.push_str(&format!("@{}({})[{}];", name, args.join(","), body_cleaned));
            }
            NST::Var(v) => {
                let var_type = match v.value {
                    VVal::Str(_) => "s",
                    VVal::Int(_) => "i",
                    VVal::F(_) => "f",
                    VVal::VarRef(_, _) => "v",
                };

                let var_value = match &v.value {
                    VVal::Str(t) => t,
                    VVal::Int(t) => &t.to_string(),
                    VVal::F(t) => &t.to_string(),
                    VVal::VarRef(_, t) => t,
                };

                bc.push_str(&format!(".{}!{}!{};", v.name, var_type, var_value));
            }
            NST::Input(v) => {
                bc.push_str(&format!("..{};", v)); // Input command with separator
            }
            NST::WAIT(t) => {
                println!("{}", format!("Waiting for {} seconds...", t).cyan());
                sleep(Duration::from_millis({ *t } * 1000));
            }
            _ => {}
        }
    }

    bc
}

fn str2bin(input: &str) -> String {
    input
        .chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<String>>()
        .join("=")
}
fn _bin2str(input: &str) -> String {
    input
        .split('=')
        .filter_map(|bin| u8::from_str_radix(bin, 2).ok())
        .map(char::from)
        .collect()
}
