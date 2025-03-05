use crate::{
    err_system::err_types::ErrTypes, helpers::condition_parser::parse_condition, parse_systems::parse, tok_system::tokens::Token
};
use super::{parse4::parse4, AST};

#[allow(unused)]
pub fn parse3(
    token: &Token,
    token_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    ast: &mut Vec<AST>,
    code: &String,
    collected_vars: &mut Vec<(String, &'static str)>,
    collected_errors: &mut Vec<ErrTypes>,
    line: &mut i32,
) {
    match token {
        Token::Iden(iden) if iden == "while" => {
            let mut collected_code = String::new();
            let mut cond = Vec::new();
            let mut body = Vec::new();

            while let Some(tok) = token_iter.next() {
                if tok == &Token::LCurly {
                    break;
                } else if tok == &Token::EOL {
                    *line += 1;
                    continue;
                } else {
                    if let Some(line_str) = code.lines().nth((*line - 1) as usize) {
                        collected_code.push_str(line_str);
                        collected_code.push('\n');
                    }
                    cond.push(tok.clone());
                }
            }
            let parsed_cond = parse_condition(&cond, collected_errors, collected_vars, *line);

            while let Some(tok) = token_iter.next() {
                if tok == &Token::RCurly {
                    break;
                } else if tok == &Token::EOL {
                    *line += 1;
                    body.push(tok.clone());
                    continue;
                } else {
                    body.push(tok.clone());
                }
            }
            let body = parse(&body, &collected_code, "",true, collected_vars, collected_errors);
            ast.push(AST::While(body.0, parsed_cond));
        }
        Token::Iden(iden) if iden == "if" => {
            let mut collected_code = String::new();
            let mut cond = Vec::new();
            let mut body = Vec::new();

            while let Some(tok) = token_iter.next() {
                if tok == &Token::LCurly {
                    break;
                } else if tok == &Token::EOL {
                    *line += 1;
                    continue;
                } else {
                    if let Some(line_str) = code.lines().nth((*line - 1) as usize) {
                        collected_code.push_str(line_str);
                        collected_code.push('\n');
                    }
                    cond.push(tok.clone());
                }
            }
            let parsed_cond = parse_condition(&cond, collected_errors, collected_vars, *line);

            while let Some(tok) = token_iter.next() {
                if tok == &Token::RCurly {
                    break;
                } else if tok == &Token::EOL {
                    *line += 1;
                    body.push(tok.clone());
                    continue;
                } else {
                    body.push(tok.clone());
                }
            }
            let body = parse(&body, &collected_code, "",true,collected_vars, collected_errors);
            ast.push(AST::IF(body.0, parsed_cond));
        }
        _ => {
            parse4(token, token_iter, ast, code, collected_vars, collected_errors, line);
        }
    }
}
