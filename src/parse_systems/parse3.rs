use crate::{err_system::err_types::ErrTypes, helpers::condition_parser::parse_condition, parse_systems::parse, tok_system::tokens::Token};
use super::{AST, COLLECTED_ERRORS, LINE};

#[allow(unused,dead_code,unreachable_code,unreachable_patterns)]
pub fn parse3(token: &Token, token_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>, ast: &mut Vec<AST>, code: &String) {
    match token {
        Token::Iden(iden) if iden == "while" => {
            let mut collected_code = String::new();
            let mut cond = Vec::new();
            let mut body = Vec::new();
            while let Some(tok) = token_iter.next() {
                if tok == &Token::LCurly {
                    break;
                } else if tok == &Token::EOL {
                    unsafe { LINE += 1 };
                    continue;
                } else {
                    collected_code.push_str(&code.lines().nth(unsafe { (LINE - 1).try_into().unwrap() }).unwrap());
                    collected_code.push('\n');
                    cond.push(tok.clone());
                }
            }
            let parsed_cond = parse_condition(&cond);
            while let Some(tok) = token_iter.next() {
                if tok == &Token::RCurly {
                    break;
                } else {
                    body.push(tok.clone());
                }
            }
            let body = parse(&body, &collected_code);
            ast.push(AST::While(body, parsed_cond));
        }
        Token::Iden(iden) if iden == "if" => {
            let mut collected_code = String::new();
            let mut cond = Vec::new();
            let mut body = Vec::new();
            while let Some(tok) = token_iter.next() {
                if tok == &Token::LCurly {
                    break;
                } else if tok == &Token::EOL {
                    unsafe { LINE += 1 };
                    continue;
                } else {
                    collected_code.push_str(&code.lines().nth(unsafe { (LINE - 1).try_into().unwrap() }).unwrap());
                    collected_code.push('\n');
                    cond.push(tok.clone());
                }
            }
            let parsed_cond = parse_condition(&cond);
            while let Some(tok) = token_iter.next() {
                if tok == &Token::RCurly {
                    break;
                } else {
                    body.push(tok.clone());
                }
            }
            let body = parse(&body, &collected_code);
            ast.push(AST::IF(body, parsed_cond));
        }
        _ => {
            //println!("├──[!!] All parsers failed!");
            if let Ok(mut errors) = COLLECTED_ERRORS.lock() {
                unsafe { errors.push(ErrTypes::UnexpectedToken(LINE)) };
            }
        }
    }
}
