use std::process::exit;

use colored::Colorize;
use parse1::p1;
use crate::{
    err_system::{err_types::ErrTypes, error_msg_gen::gen_error_msg}, helpers::Condition, optimisers::pass1::pass1, tok_system::tokens::Token
};

#[derive(Debug)]
pub enum AST {
    Print {
        descriptor: FileDescriptors,
        text: Vec<PrintTokTypes>,
    },
    Var(Variables),
    While(Vec<AST>, Condition),
    IF(Vec<AST>, Condition),
    VarAssign(Variables),
}

#[derive(Debug)]
/// Different types of print tokens, e.g. variables, words, spaces, and newlines.
pub enum PrintTokTypes {
    Var(String),
    Newline,
    Word(String),
    Space,
}

#[derive(Debug, Clone, Copy)]
/// File descriptors.
pub enum FileDescriptors {
    STDOUT = 1,
    STDERR = 2,
    STDIN = 0,
}

impl FileDescriptors {
    pub fn display(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Variable types.
pub enum Variables {
    I32(&'static str, i32),
    I8(&'static str, i8),
    I16(&'static str, i16),
    I64(&'static str, i64),
    Char(&'static str, char),
    Str(&'static str, String),
    F32(&'static str, f32),
    F64(&'static str, f64),
    // First is the variable name, second is the reference variable name.
    REF(&'static str, String),
    // Variable holding mathematical operations.
    MATH(String, String),
}

pub mod parse1;
pub mod parse2;
pub mod parse3;
pub mod parse4;
pub mod parse5;

/// Parses tokens into an AST while collecting variables and reporting errors.
/// 
/// # Arguments
/// - `tokens`: The tokens to parse.
/// - `code`: The source code (for error messages).
/// - `file`: Name of the file being parsed.
/// - `use_args_vars_err`: If `true`, the function uses the provided vectors without clearing them;
///   if `false`, it clears the provided vectors before parsing.
/// - `collected_vars`: A mutable reference to a vector of variable tuples (name and type).
/// - `collected_errors`: A mutable reference to a vector of errors.
/// 
/// # Returns
/// A triple containing:
/// - The parsed AST (owned),
/// - A reference to the collected variables,
/// - A reference to the collected errors.
pub fn parse<'a>(
    tokens: &'a Vec<Token>,
    code: &String,
    file: &'static str,
    use_args_vars_err: bool,
    collected_vars: &'a mut Vec<(String, &'static str)>,
    collected_errors: &'a mut Vec<ErrTypes>,
    line : i32,
) -> (Vec<AST>, &'a Vec<(String, &'static str)>, &'a Vec<ErrTypes>) {

    if !use_args_vars_err {
        collected_vars.clear();
        collected_errors.clear();
    }
    let tpcode = code.clone();
    let mut line = line;
    let mut ast = p1(tokens, &tpcode, collected_errors, collected_vars, &mut line);
    pass1(&mut ast);
    
    if !collected_errors.is_empty() {
        println!("{}{}", "[!] Errors in file ".bold().red(), file);
        for err in collected_errors.iter() {
            println!("{}\n──+++++++++++++++──", gen_error_msg(*err, code));
        }
        eprintln!("{}", "[!]".bold().red());
        exit(1);
    }
    
    (ast, collected_vars, collected_errors)
}
