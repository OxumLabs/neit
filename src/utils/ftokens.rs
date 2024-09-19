use super::{
    tokens::{print::process_print, var::process_var},
    types::{fvars, Args, Tokens},
};

#[allow(unused)]
pub fn parse_single_line(
    line: &str,
    line_number: usize,
    p_label: &mut i32,
    lv: &mut Vec<fvars>,
) -> Result<Tokens, String> {
    //let mut p_label = 0;
    let line = line.trim();
    if line.trim().is_empty() {
        return Err("|_EMP_|".to_string());
    }

    // Return an error if a function declaration is encountered.
    if line.trim().starts_with("fn ") || line.trim().starts_with("pub fn") {
        return Err(format!(
            "Error at line {}: Function declarations are not allowed here.\n\
            Hint: Function declarations should not be placed in this context.\n\
            Code:\n   => {}",
            line_number, line
        ));
    }

    // Process print statements
    if line.starts_with("_WRT(") && line.ends_with(")") {
        let txt = line[5..].trim_end_matches(")");
        *p_label += *p_label + 365;
        let print_token = process_print(p_label, txt);
        return Ok(print_token);
    } else if line.starts_with("may ") && line.contains("=") {
        let vr = process_var(line);
        match vr {
            Ok(vr) => {
                return Ok(Tokens::Var(vr.0, vr.1));
            }
            Err(e) => return Err(e),
        }
    }

    // Handle function calls (not declarations)
    let args: Vec<&str> = line.trim().split('(').collect();
    if args.len() == 2 {
        let (nm, args_str) = (
            args.first().unwrap(),
            args.get(1).unwrap().trim_end_matches(')'),
        );

        // Remove any empty arguments
        let provided_args: Vec<&str> = args_str
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        // Simulate finding the function in existing tokens (using hardcoded expected args)
        let expected_args: Vec<Args> = vec![]; // You'd fetch this from pre-existing function definitions

        if provided_args.len() != expected_args.len() {
            return Err(format!(
                "Error at line {}: Function '{}' called with incorrect number of arguments.\n\
                Hint: Expected {} arguments but got {}.\n\
                Code:\n   => {}",
                line_number,
                nm,
                expected_args.len(),
                provided_args.len(),
                line
            ));
        }

        for (provided, expected) in provided_args.iter().zip(expected_args.iter()) {
            let provided_type = match determine_type(provided) {
                Ok(t) => t,
                Err(e) => {
                    return Err(format!(
                        "Error at line {}: Argument '{}' could not be parsed. {}\n\
                        Hint: Ensure arguments are of correct type.\n\
                        Code:\n   => {}",
                        line_number, provided, e, line
                    ));
                }
            };

            let expected_type = match expected {
                Args::Str(_) => "string",
                Args::Int(_) => "int",
                Args::Float(_) => "float",
                _ => "unknown",
            };

            if provided_type != expected_type {
                return Err(format!(
                    "Error at line {}: Argument type mismatch in function call '{}'.\n\
                    Hint: Expected argument type '{}' but got '{}'.\n\
                    Code:\n   => {}",
                    line_number, nm, expected_type, provided_type, line
                ));
            }
        }

        return Ok(Tokens::FnCall(nm.to_string()));
    }

    Err(format!(
        "Error at line {}: Could not parse the provided line.\n\
        Hint: Ensure the code syntax is correct.\n\
        Code:\n   => {}",
        line_number, line
    ))
}

fn determine_type(arg: &str) -> Result<&'static str, String> {
    let trimmed = arg.trim();

    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        Ok("string")
    } else if trimmed.starts_with('\'') && trimmed.ends_with('\'') {
        Ok("string")
    } else if trimmed.parse::<i32>().is_ok() {
        Ok("int")
    } else if trimmed.parse::<f64>().is_ok() {
        Ok("float")
    } else {
        Err(format!("Could not determine type for argument: {}", arg))
    }
}
