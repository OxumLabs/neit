use std::process::exit;

use crate::{err_system::err_types::ErrTypes, tok_system::tokens::Token};
use super::{ERRORS, FileDescriptors, PrintTokTypes, AST, LINE};

#[allow(unused)]
pub fn p1(tokens: &[Token]) -> Vec<AST> {
    let mut ast = Vec::new();
    let mut tokens_iter = tokens.iter().peekable();
    
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Iden(cmd) => {
                if cmd == "print" || cmd == "eprint" || cmd == "println" || cmd == "eprintln" {
                    let fd = if cmd == "eprint"||cmd == "eprintln" { FileDescriptors::STDERR } else { FileDescriptors::STDOUT };
                    let add_newline = (cmd == "println" || cmd == "eprintln" );
                    let mut has_seen_delim_space = false;
                    let mut content = Vec::new();
                    let mut escape_mode = false;
                    
                    while let Some(next_tok) = tokens_iter.peek() {
                        match next_tok {
                            Token::EOL | Token::EOF => {
                                tokens_iter.next();
                                unsafe { LINE += 1 };
                                if add_newline {
                                    content.push(PrintTokTypes::Newline);
                                }
                                ast.push(AST::Print { descriptor: fd, text: content });
                                content = Vec::new();
                                break;
                            }
                            Token::Space => {
                                tokens_iter.next();
                                if !has_seen_delim_space {
                                    has_seen_delim_space = true;
                                } else {
                                    content.push(PrintTokTypes::Space);
                                }
                            }
                            Token::BackSlash => {
                                tokens_iter.next();
                                escape_mode = true;
                            }
                            Token::Iden(text) => {
                                tokens_iter.next();
                                if escape_mode {
                                    if text == "n" {
                                        content.push(PrintTokTypes::Newline);
                                    } else if text == "%" {
                                        if let Some(Token::Iden(var_text)) = tokens_iter.peek() {
                                            let var_token = var_text.clone();
                                            tokens_iter.next();
                                            let mut var_collected = String::new();
                                            for c in var_token.chars() {
                                                if c.is_alphanumeric() || c == '_' {
                                                    var_collected.push(c);
                                                } else {
                                                    break;
                                                }
                                            }
                                            content.push(PrintTokTypes::Var(var_collected));
                                        }
                                    } else {
                                        content.push(PrintTokTypes::Word(format!("\\{}", text)));
                                    }
                                    escape_mode = false;
                                } else {
                                    content.push(PrintTokTypes::Word(text.clone()));
                                }
                            }
                            _ => {
                                tokens_iter.next();
                            }
                        }
                    }
                }
                else{
                    //give err
                    unsafe {match ERRORS.lock(){
                        Ok(mut e) => {
                            e.push(ErrTypes::UnknownCMD(LINE));
                        },
                        Err(_) => {
                            eprintln!("Error: Failed to lock mutex");
                            exit(1);
                        }
                    }};

                }
            }
            Token::EOL => {
                unsafe { LINE += 1 };
            }
            _ => {

            }
        }
    }
    
    ast
}