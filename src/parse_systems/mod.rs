use std::process::exit;

use colored::Colorize;
use parse1::p1;

use crate::{err_system::{self, err_types::ErrTypes}, tok_system::tokens::Token};

pub static mut LINE: i32 = 0;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ERRORS: std::sync::Mutex<Vec<ErrTypes>> = std::sync::Mutex::new(Vec::new());
}

#[derive(Debug)]
pub enum AST{
    Print{descriptor : FileDescriptors , text : Vec<PrintTokTypes>}
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
pub mod parse1;

pub fn parse(tokens : &Vec<Token>,code : &String) -> Vec<AST> {
    let ast = p1(tokens);
    match ERRORS.lock() {
        Ok(mut errors) => {
            if !errors.is_empty() {
                for err in errors.iter() {
                    eprintln!("{}",err_system::error_msg_gen::gen_error_msg(*err,code).red());
                }
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("{}",e);
            exit(1);
        }
    }
    ast
}