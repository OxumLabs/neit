use super::{parse2::parse2, FileDescriptors, PrintTokTypes, AST, LINE};
use crate::tok_system::tokens::Token;

pub fn p1(tokens: &[Token], code: &String) -> Vec<AST> {
    let mut ast = Vec::new();
    let mut tokens_iter = tokens.iter().peekable();
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Iden(cmd) if cmd == "print" || cmd == "println" || cmd == "eprint" || cmd == "eprintln" => {
                    let fd = if cmd == "eprint" || cmd == "eprintln" {
                        FileDescriptors::STDERR
                    } else {
                        FileDescriptors::STDOUT
                    };
                    let add_newline = cmd == "println" || cmd == "eprintln";
                    let mut content = Vec::new();
                    content.reserve(16);
                    let mut escape_mode = false;
                    let mut has_seen_delim_space = false;
                    loop {
                        match tokens_iter.next() {
                            Some(Token::EOL) | Some(Token::EOF) => {
                                unsafe { LINE += 1 };
                                if add_newline {
                                    content.push(PrintTokTypes::Newline);
                                }
                                ast.push(AST::Print {
                                    descriptor: fd,
                                    text: content,
                                });
                                break;
                            }
                            Some(Token::Space) => {
                                if !has_seen_delim_space {
                                    has_seen_delim_space = true;
                                } else {
                                    content.push(PrintTokTypes::Space);
                                }
                            }
                            Some(Token::BackSlash) => {
                                escape_mode = true;
                            }
                            Some(Token::PercentSign) => {
                                if let Some(Token::Iden(var_text)) = tokens_iter.next() {
                                    content.push(PrintTokTypes::Var(var_text.clone()));
                                } else {
                                    content.push(PrintTokTypes::Word("%".to_string()));
                                }
                            }
                            Some(Token::Iden(text)) => {
                                if escape_mode {
                                    if text == "n" {
                                        content.push(PrintTokTypes::Newline);
                                    } else {
                                        content.push(PrintTokTypes::Word(format!("\\{}", text)));
                                    }
                                    escape_mode = false;
                                } else {
                                    content.push(PrintTokTypes::Word(text.clone()));
                                }
                            }
                            Some(_) => {}
                            None => break,
                        }
                    }
                
            }
            Token::EOL => {
                unsafe { LINE += 1 };
            }
            Token::Space => {}
            _ => {
                parse2(token, &mut tokens_iter, &mut ast, code);
            }
        }
    }
    ast
}
