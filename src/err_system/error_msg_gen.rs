use colored::Colorize;

use super::err_types::ErrTypes;

pub fn gen_error_msg(err_type: ErrTypes,code : &String) -> String {
    match err_type {
        ErrTypes::UnknownCMD(line) => {
            let code_snippet = code.lines().nth((line-1) as usize).unwrap_or("Line not found");
            format!(
                "Unknown command at line {}.\n\
                 Code Snippet:\n\
                   {}\n\
                 Hint: Check the command and try again",
                line,
                code_snippet.green().bold()
            )
            
        }
        ErrTypes::UnsupportedVarType(line) => {
            let code_snippet = code.lines().nth((line-1) as usize).unwrap_or("Line not found");
            format!(
                "Unknown variable at line {}.\n\
                 Code Snippet:\n\
                   {}\n\
                 Hint: Check the variable and try again",
                line,
                code_snippet.green().bold()
            )
        }
        ErrTypes::VarAlreadyExists(line) => {
            let code_snippet = code.lines().nth((line-1) as usize).unwrap_or("Line not found");
            format!(
                "Variable already exists; error at line {}.\n\
                 Code Snippet:\n\
                   {}\n\
                 Hint: Check the variable and try again",
                line,
                code_snippet.green().bold()
            )
        }
        ErrTypes::CharVarLen(line) => {
            let code_snippet = code.lines().nth((line-1) as usize).unwrap_or("Line not found");
            format!(
                "Char variable length at line {}.\n\
                 Code Snippet:\n\
                   {}\n\
                 Hint: Check the variable and try again",
                line,
                code_snippet.green().bold()
            )
        }
        ErrTypes::InvalidMathUsage(line) => {
            let code_snippet = code.lines().nth((line-1) as usize).unwrap_or("Line not found");
            format!(
                "Invalid math usage at line {}.\n\
                 Code Snippet:\n\
                   {}\n\
                 Hint: Check the math and try again",
                line,
                code_snippet.green().bold()
            )
        }
    }
}