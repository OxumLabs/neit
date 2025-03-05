use super::{parse2::parse2, FileDescriptors, PrintTokTypes, AST};
use crate::{err_system::err_types::ErrTypes, tok_system::tokens::Token};

#[inline(always)]
#[allow(non_snake_case)]
pub fn p1(
    tokens: &[Token],
    code: &String,
    COLLECTED_ERRORS: &mut Vec<ErrTypes>,
    COLLECTED_VARS: &mut Vec<(String, &'static str)>,
    LINE: &mut i32,
) -> Vec<AST> {
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
                let mut content = Vec::with_capacity(16);
                let mut escape_mode = false;
                let mut has_seen_delim_space = false;
                loop {
                    let next_token = tokens_iter.next();
                    match next_token {
                        Some(Token::EOL) | Some(Token::EOF) => {
                            *LINE += 1;
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
                        // For any other token type, add its character representation.
                        Some(other) => {
                            let repr = match other {
                                Token::Quote          => "\"",
                                Token::EqSign         => "=",
                                Token::ADDOP          => "+",
                                Token::SUBOP          => "-",
                                Token::DIVOP          => "/",
                                Token::MULTIOP        => "*",
                                Token::DoubleEqSign   => "==",
                                Token::LCurly         => "{",
                                Token::RCurly         => "}",
                                Token::And            => "&",
                                Token::Or             => "|",
                                Token::Not            => "!",
                                Token::GreaterThan    => ">",
                                Token::LessThan       => "<",
                                Token::LSmallBrac     => "(",
                                Token::RSmallBracket  => ")",
                                _                     => "",
                            };
                            if !repr.is_empty() {
                                content.push(PrintTokTypes::Word(repr.to_string()));
                            }
                        }
                        None => break,
                    }
                }
            }
            Token::EOL => {
                *LINE += 1;
                break;
            }
            Token::Space => {}
            _ => {
                parse2(token, &mut tokens_iter, &mut ast, code, COLLECTED_VARS, COLLECTED_ERRORS, LINE);
            }
        }
    }
    ast
}
