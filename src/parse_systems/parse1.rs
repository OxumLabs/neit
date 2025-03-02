use crate::tok_system::tokens::Token;
use super::{parse2::parse2, FileDescriptors, PrintTokTypes, AST, LINE};

pub fn p1(tokens: &[Token]) -> Vec<AST> {
    let mut ast = Vec::new();
    let mut tokens_iter = tokens.iter().peekable();
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Iden(cmd) => {
                if cmd == "print" || cmd == "eprint" || cmd == "println" || cmd == "eprintln" {
                    let fd = if cmd == "eprint" || cmd == "eprintln" {
                        FileDescriptors::STDERR
                    } else {
                        FileDescriptors::STDOUT
                    };
                    let add_newline = cmd == "println" || cmd == "eprintln";
                    let mut content = Vec::new();
                    let mut escape_mode = false;
                    let mut has_seen_delim_space = false;
                    while let Some(next_tok) = tokens_iter.peek() {
                        match next_tok {
                            Token::EOL | Token::EOF => {
                                tokens_iter.next();
                                unsafe { LINE += 1 };
                                if add_newline {
                                    content.push(PrintTokTypes::Newline);
                                }
                                ast.push(AST::Print { descriptor: fd, text: content });
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
                            Token::PercentSign => {
                                tokens_iter.next();
                                if let Some(Token::Iden(var_text)) = tokens_iter.peek() {
                                    tokens_iter.next();
                                    content.push(PrintTokTypes::Var(var_text.clone()));
                                } else {
                                    content.push(PrintTokTypes::Word("%".to_string()));
                                }
                            }
                            Token::Iden(text) => {
                                tokens_iter.next();
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
                            _ => {
                                tokens_iter.next();
                            }
                        }
                    }
                } else {
                    parse2(token, &mut tokens_iter, &mut ast);
                }
            }
            Token::EOL => {
                unsafe { LINE += 1 };
            }
            _ => {}
        }
    }
    ast
}
