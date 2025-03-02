use std::process::exit;

use colored::Colorize;
use parse1::p1;

use crate::{err_system::{self, err_types::ErrTypes}, tok_system::tokens::Token};

pub static mut LINE: i32 = 1;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref COLLECTED_ERRORS: std::sync::Mutex<Vec<ErrTypes>> = std::sync::Mutex::new(Vec::new());
    pub static ref COLLECTED_VARS: std::sync::Mutex<Vec<(String,&'static str)>> = std::sync::Mutex::new(Vec::new());
}

#[derive(Debug)]
pub enum AST{
    Print{descriptor : FileDescriptors , text : Vec<PrintTokTypes>},
    Var(Variables),
}
#[derive(Debug)]

///diff types for the print tokens that it will store like newline or variable and all
pub enum PrintTokTypes{
    Var(String),//later take in variable enum to ensure types easily
    Newline,
    Word(String),
    Space,
}
#[derive(Debug,Clone,Copy)]

///file descriptors
pub enum FileDescriptors{
    STDOUT = 1,
    STDERR = 2,
    STDIN = 0
}
impl FileDescriptors{
    pub fn display(self) -> i32 {
        self as i32
    }
}

#[derive(Debug,PartialEq,Clone)]
///Variables enums
pub enum Variables{
    I32(&'static str , i32),
    I8(&'static str , i8),
    I16(&'static str , i16),
    I64(&'static str , i64),
    Char(&'static str , char),
    F32(&'static str , f32),
    F64(&'static str , f64),
    // first is the name of the variable, second is the ref var name
    REF(&'static str, String),
    //variable containing maths operations
    MATH(String, String),
}

pub mod parse1;
pub mod parse2;

pub fn parse(tokens : &Vec<Token>,code : &String) -> Vec<AST> {
    let ast = p1(tokens);
    match COLLECTED_ERRORS.lock() {
        Ok(errors) => {
            if !errors.is_empty() {
            eprintln!("{}", "┌[ERRORS]".bold().red());
            for err in errors.iter() {
                eprintln!("{}", format!("++++++++++++++++++++++++++++++++++++++++\n{}\n+++++++++++++++++++++++++++++++++++++++++++", err_system::error_msg_gen::gen_error_msg(*err, code)).red());
            }
            exit(1);
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
    // println!("{}", "┌[AST]".bold().green());
    // for ast in ast.iter() {
    //     println!("{:?}", ast);
    // }
    // println!("{}", "└[AST]".bold().green());
    ast
}