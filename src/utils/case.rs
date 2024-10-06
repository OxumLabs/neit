use super::{
    token::determine_type,
    tokens::{input::process_input, print::process_print, var::process_var},
    types::{Args, Tokens},
};
use colored::*; // Import the colored crate

pub fn process_case(
    ln: &str,
    code: Vec<String>,
    index: &mut i64,
    _fc: bool,
) -> Result<Vec<Tokens>, String> {
    // Begin processing case
    let mut brace_depth = 0;
    let mut case_body = vec![ln.to_string()]; // Start the case body with the case line
    let mut case_tokens = Vec::new(); // Create a new token list for the case

    brace_depth += ln.matches('{').count();
    if brace_depth != 0 {
        // Fixed the logical condition
        brace_depth -= ln.matches('}').count();
    }

    // Process the case body until the corresponding closing brace
    while *index < code.len().try_into().unwrap() {
        *index += 1; // Move to the next line
        let next_line = code[*index as usize].trim();

        if next_line.is_empty() {
            continue; // Skip empty lines
        }

        case_body.push(next_line.to_string());
        brace_depth += next_line.matches('{').count();
        brace_depth -= next_line.matches('}').count();

        if brace_depth == 0 {
            // All braces are closed, process the case
            let full_case_code = case_body.join("\n");

            // Process each line in the case body
            for (line_index, line) in full_case_code.lines().enumerate() {
                let line_number = line_index + 1; // Adjust for the line index
                let ln = line.trim();

                if ln.starts_with("pub fn") || ln.starts_with("fn ") {
                    // Handle function definitions
                    if ln.ends_with("{") {
                        // Function declaration
                        return Err(format!(
                            "{} Oh no, rookie move! Found another function at line {} while you're still inside one.\n\
                             → First finish what you started before moving on!\n\
                             Code:\n   => {}\n\
                             Seriously, let’s close that function off before we get ahead of ourselves, okay?",
                            "✘".red(), line_number, ln
                        ));
                    }
                } else if ln.starts_with("case ") && ln.ends_with("{") {
                    // Check for nested cases
                    return Err(format!(
                        "{} Found nested case declaration at line {}. Case statements can't be nested!\n\
                         Code:\n   => {}",
                        "✘".red(), line_number, ln
                    ));
                } else if ln.trim().starts_with("may") && ln.contains('=') {
                    // Handle variable declarations
                    let var_result = process_var(ln.trim(), &case_tokens, true);
                    match var_result {
                        Ok(var) => {
                            case_tokens.push(Tokens::Var(var.0, var.1, true)); // Add to case tokens
                        }
                        Err(e) => return Err(e),
                    }
                } else if ln.trim().starts_with("must ") {
                    // Handle required variable declarations
                    let var_result = process_var(ln.trim(), &case_tokens, false);
                    match var_result {
                        Ok(var) => {
                            case_tokens.push(Tokens::Var(var.0, var.1, false)); // Add to case tokens
                        }
                        Err(e) => return Err(e),
                    }
                } else if ln.trim().starts_with("print(") && ln.trim().ends_with(")") {
                    // Handle print statements
                    let txt: String = ln[6..ln.len() - 1].trim().to_string(); // Extract print arguments

                    let ptxt = process_print(&mut 0, &txt, &case_tokens); // Changed p_label to 0 since we don't need it globally
                    case_tokens.push(ptxt); // Add to case tokens
                } else if ln.starts_with("takein(") {
                    // Handle input statements
                    let tkn = process_input(&ln, &case_tokens);
                    match tkn {
                        Ok(tkn) => {
                            case_tokens.push(tkn); // Add to case tokens
                        }
                        Err(e) => return Err(e),
                    }
                } else {
                    // Handle function calls or other statements
                    let args: Vec<&str> = ln.trim().split('(').collect();
                    let mut found_function = false;

                    if args.len() == 2 {
                        let (nm, args_str) = (
                            args.first().unwrap(),
                            args.get(1).unwrap().trim_end_matches(')'),
                        );

                        let provided_args: Vec<String> = args_str
                            .split(',')
                            .map(|s| s.trim().to_string()) // Convert &str to String after trimming
                            .filter(|s| !s.is_empty()) // Filter out empty strings
                            .collect();

                        if let Some(Tokens::Func(f)) = case_tokens
                            .iter()
                            .find(|tkn| matches!(tkn, Tokens::Func(f) if f.name == *nm.trim()))
                        {
                            let expected_args: Vec<Args> = f.args.clone();

                            if provided_args.len() != expected_args.len() {
                                return Err(format!(
                                    "{} Oops! Looks like you called the function '{}' at line {} with the wrong number of arguments.\n\
                                     → Expected {}, but you gave me {}. Rookie mistake, right?\n\
                                     Code:\n   => {}\n\
                                     Let’s fix that up, shall we?",
                                    "✘".red(), nm.trim(), line_number, expected_args.len(), provided_args.len(), ln
                                ));
                            }

                            for (provided, expected) in
                                provided_args.iter().zip(expected_args.iter())
                            {
                                let provided_type =
                                    match determine_type(provided, &case_tokens) {
                                        Ok(t) => t,
                                        Err(_) => {
                                            return Err(format!(
                                            "{} Are you kidding me? I can't even parse '{}' at line {}.\n\
                                         → Double check that argument—I'm begging you!\n\
                                         Code:\n   => {}",
                                            "✘".red(), provided, line_number, ln
                                        ))
                                        }
                                    };

                                let expected_type = match expected {
                                    Args::Str(_) => "string",
                                    Args::Int(_) => "int",
                                    Args::Float(_) => "float",
                                    _ => "unknown",
                                };

                                if provided_type != expected_type
                                    && !(provided_type == "float" && expected_type == "int")
                                    && !(provided_type == "int" && expected_type == "float")
                                {
                                    return Err(format!(
                                        "{} Come on! You passed a '{}' to function '{}' expecting a '{}'. At line {}.\n\
                                         → That's not how this works! Fix it!\n\
                                         Code:\n   => {}",
                                        "✘".red(), provided_type, nm.trim(), expected_type, line_number, ln
                                    ));
                                }
                            }
                            found_function = true;
                        }
                    }
                    if !found_function {
                        if ln.chars().all(|c| c == '}') {
                            return Ok(case_tokens); // Return case tokens if closing brace found
                        }
                        return Err(format!(
                            "{} Error: The function '{}' is not defined. At line {}.\n\
                             → That’s a real head-scratcher! Check your function definitions, please.\n\
                             Code:\n   => {}",
                            "✘".red(),
                            ln,
                            line_number,
                            ln
                        ));
                    }
                }
            }

            // Exit the loop once processed
            break;
        }
    }

    Ok(case_tokens) // Return the tokens generated within the case
}
