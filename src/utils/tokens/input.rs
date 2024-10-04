use crate::utils::types::{Tokens, Vars};

pub fn process_input(ln: &str, vars: &[Tokens]) -> Result<Tokens, String> {
    // Extract the variable name from the input line
    let vrname = ln[7..].trim_end_matches(")");

    // Iterate through the provided variables to find a match
    for token in vars {
        match token {
            Tokens::Var(var_type, name, is_mutable) => {
                // Check if the current variable matches the requested variable name
                if *name == vrname {
                    // Check if the variable is mutable
                    if !is_mutable {
                        return Err(format!(
                            "✘ Error: Trying to modify the static variable '{}'.\n\
                             → Note: Static variables are immutable by default.\n\
                             ⚙ Suggestion: Use 'may {} = <value>' for mutable variables.",
                            name, name
                        ));
                    }

                    // If the variable is a string type, return the corresponding token
                    match var_type {
                        Vars::STR(_) => {
                            return Ok(Tokens::In(vrname.to_string())); // Return the token if it's a string
                        }
                        _ => {
                            return Err(format!(
                                "✘ Error: Expected a string for '{}', but received a {}.\n\
                                 ⚙ Note: Make sure the type is correct.",
                                name,
                                match var_type {
                                    Vars::INT(_) => "integer",
                                    Vars::F(_) => "float",
                                    _ => "unknown type",
                                }
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Return an error if the variable was not found
    Err(format!(
        "✘ Error: Variable '{}' not found.\n\
         → Note: Check if it was declared correctly.",
        vrname
    ))
}
