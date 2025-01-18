use crate::{
    err::ErrT,
    lex::{lex, TokType, Tokens},
    p::{parse, VVal, NST},
    p2::parse_condition,
};
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
};

#[allow(unused)]
pub fn p3(
    tok: &Tokens,
    tokiter: &mut std::iter::Peekable<std::slice::Iter<'_, Tokens>>,
    codes: &[&str],
    errors: &mut Vec<ErrT>,
    nst: &mut Vec<NST>,
    ln: &mut usize,
    vars: &HashMap<String, VVal>,
    file: &str,
) {
    //println!("toks in p3 : {:?}",tok);

    match (tok.get_type(), tok.get_value()) {
        (TokType::CMD, "bring") => {
            //println!("[DEBUG] Processing bring command");
            let mut path = String::new();
            let debug_enabled = 0; // Set to 0 to disable debug messages
        
            while let Some(args) = tokiter.next() {
                if debug_enabled != 0 {
                    println!("[DEBUG] Token: {:?} = {}", args.get_type(), args.get_value());
                }
        
                if args.get_type() == TokType::OP && args.get_value() == "," {
                    if debug_enabled != 0 {
                        println!("[DEBUG] Processing path: {}", path.trim());
                    }
        
                    if path.trim() == file {
                        errors.push(ErrT::InvFILE(*ln, path.clone()));
                        break;
                    }
        
                    match File::open(path.trim()) {
                        Ok(_) => {
                            match read_to_string(path.trim()) {
                                Ok(code) => {
                                    if debug_enabled != 0 {
                                        println!("[DEBUG] File read: {} bytes", code.len());
                                    }
        
                                    let mut toks = Tokens::new();
                                    lex(&code, &mut toks);
        
                                    if debug_enabled != 0 {
                                        println!("[DEBUG] Lexed tokens: {}", toks.len());
                                    }
        
                                    let mut inst = parse(
                                        &toks,
                                        &code.split("\n").collect::<Vec<&str>>(),
                                        &path,
                                        false,
                                        errors,
                                    );
        
                                    if debug_enabled != 0 {
                                        println!("[DEBUG] Parsed instructions: {}", inst.len());
                                    }
        
                                    nst.extend(inst);
                                }
                                Err(e) => {
                                    if debug_enabled != 0 {
                                        println!("[DEBUG] File read error: {}", e);
                                    }
                                    errors.push(ErrT::InvFILE(*ln, path.clone()));
                                }
                            }
                        }
                        Err(e) => {
                            if debug_enabled != 0 {
                                println!("[DEBUG] File open error: {}", e);
                            }
                            errors.push(ErrT::InvFILE(*ln, path.clone()));
                        }
                    }
        
                    path.clear();
                } else if args.get_type() == TokType::EOL {
                    if debug_enabled != 0 {
                        println!("[DEBUG] EOL reached, processing final path: {}", path.trim());
                    }
        
                    if path.trim() == file {
                        errors.push(ErrT::InvFILE(*ln, path.clone()));
                        break;
                    }
        
                    match File::open(path.trim()) {
                        Ok(_) => {
                            match read_to_string(path.trim()) {
                                Ok(code) => {
                                    if debug_enabled != 0 {
                                        println!("[DEBUG] File read: {} bytes", code.len());
                                    }
        
                                    let mut toks = Tokens::new();
                                    lex(&code, &mut toks);
        
                                    if debug_enabled != 0 {
                                        println!("[DEBUG] Lexed tokens: {}", toks.len());
                                    }
        
                                    let mut inst = parse(
                                        &toks,
                                        &code.split("\n").collect::<Vec<&str>>(),
                                        &path,
                                        false,
                                        errors,
                                    );
        
                                    if debug_enabled != 0 {
                                        println!("[DEBUG] Parsed instructions: {}", inst.len());
                                    }
        
                                    nst.extend(inst);
                                }
                                Err(e) => {
                                    if debug_enabled != 0 {
                                        println!("[DEBUG] File read error: {}", e);
                                    }
                                    errors.push(ErrT::InvFILE(*ln, path.clone()));
                                }
                            }
                        }
                        Err(e) => {
                            if debug_enabled != 0 {
                                println!("[DEBUG] File open error: {}", e);
                            }
                            errors.push(ErrT::InvFILE(*ln, path.clone()));
                        }
                    }
        
                    break;
                } else {
                    path.push_str(args.get_value());
        
                    if debug_enabled != 0 {
                        println!("[DEBUG] Current path build: {}", path);
                    }
                }
            }
        
            if debug_enabled != 0 {
                println!("[DEBUG] Bring command completed");
            }
        }
        
        (TokType::CMD, "while") => {
            let mut cond = String::new();
            let mut body_tokens = Vec::new();
            let mut in_parentheses = false;
            let mut brace_count = 0;

            // Parse condition inside parentheses
            while let Some(tok) = tokiter.next() {
                match (tok.get_type(), tok.get_value()) {
                    (TokType::OP, "(") if !in_parentheses => {
                        in_parentheses = true;
                    }
                    (TokType::EOL, _) => {
                        *ln += 1;
                    }
                    (TokType::OP, ")") if in_parentheses => {
                        in_parentheses = false;
                        break; // Exit condition parsing
                    }
                    (TokType::SPACE, _) => {
                        continue; // Ignore spaces inside condition
                    }
                    (_, _) if in_parentheses => {
                        cond.push_str(tok.get_value());
                    }
                    _ => {
                        println!("unmatched paren\n\n\n");

                        errors.push(ErrT::UnmatchedParen(*ln, codes[*ln].to_string()));
                        return;
                    }
                }
            }

            // Check for unmatched or empty condition
            if in_parentheses {
                errors.push(ErrT::UnmatchedParen(*ln, codes[*ln].to_string()));
                return;
            }
            if cond.is_empty() {
                errors.push(ErrT::EmptyCond(*ln, codes[*ln].to_string()));
                return;
            }

            // Parse the condition
            let cond_parsed = parse_condition(&cond, *ln, errors, vars, nst);
            let mut condition = match cond_parsed {
                Some(cond) => cond,
                None => {
                    errors.push(ErrT::InVCond(*ln, cond.clone()));
                    return;
                }
            };
            // Parse body inside braces with brace counting
            while let Some(tok) = tokiter.next() {
                match (tok.get_type(), tok.get_value()) {
                    (TokType::OP, "{") => {
                        println!("brace count : {}", brace_count);
                        brace_count += 1; // Increment brace count
                        if brace_count == 1 {
                            continue; // Skip the first `{` to start body parsing
                        }
                    }
                    (TokType::OP, "}") => {
                        println!("brace count : {}", brace_count);
                        brace_count -= 1; // Decrement brace count
                        if brace_count == 0 {
                            break; // Exit body parsing
                        }
                    }
                    (_, _) if brace_count > 0 => {
                        body_tokens.push(tok.clone());
                    }
                    _ => {}
                }
            }

            // Check for unmatched braces
            if brace_count != 0 {
                println!("braces : {}", brace_count);
                errors.push(ErrT::UnmatchedParen(
                    *ln,
                    "Unmatched braces in while loop".to_string(),
                ));
                return;
            }

            // Check for empty body
            if body_tokens.is_empty() {
                errors.push(ErrT::InVCond(*ln, "Empty body for while loop".to_string()));
                return;
            }

            // Parse the body tokens
            let body = parse(&body_tokens, codes, file, false, errors);
            nst.push(NST::NWHILE(condition, body));
        }
        (TokType::CMD, "exit") => {
            let mut exs = String::new();
            while let Some(tok) = tokiter.next() {
                match (tok.get_type(), tok.get_value()) {
                    (TokType::EOL, _) => {
                        *ln += 1;
                        break; // Exit parsing
                    }
                    (_, _) => {
                        exs.push_str(tok.get_value());
                    }
                }
            }
            match exs.trim() {
                "" => {
                    errors.push(ErrT::InvVal(*ln, "exit".to_string(), "".to_string()));
                }
                "ok" | "success" | "0" => {
                    nst.push(NST::EX(0));
                }
                "fail" | "failure" | "1" => {
                    nst.push(NST::EX(1));
                }
                "invalid arg" | "inv arg" | "128" => {
                    nst.push(NST::EX(128));
                }
                "not found" | "nf" | "127" => {
                    nst.push(NST::EX(127));
                }
                "permission err" | "perm err" | "permission denied" | "126" => {
                    nst.push(NST::EX(126));
                }
                "killed" | "kill" | "137" => {
                    nst.push(NST::EX(137));
                }
                "interrupt" | "int" | "signal int" | "130" => {
                    nst.push(NST::EX(130));
                }
                "segfault" | "seg" | "segmentation fault" | "11" => {
                    nst.push(NST::EX(11));
                }
                "out of range" | "range error" | "255" => {
                    nst.push(NST::EX(255));
                }
                _ => {
                    errors.push(ErrT::InvVal(*ln, "exit".to_string(), exs.to_string()));
                }
            }
        }
        _ => {}
    }
}
