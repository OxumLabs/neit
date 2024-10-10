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
                            "✘ Error: Attempt to Modify Static Variable Detected\n\
                            ➔ Issue: You’re trying to modify the static variable '{}', which is immutable by default.\n\
                            ➔ Action: To change a variable's value, use a mutable variable. Declare it like this: 'let mut {} = <value>'.\n\
                            Let's correct this and use the appropriate variable type!",name,name
                        ));
                    }

                    // If the variable is a string type, return the corresponding token
                    match var_type {
                        Vars::STR(_) => {
                            return Ok(Tokens::In(vrname.to_string())); // Return the token if it's a string
                        }
                        _ => {
                            return Err(format!(
                                "✘ Error: Type Mismatch Detected\n
                                Oops! You expected a string for '{}', but instead received a {}.\n
                                ➔ What Happened: The value you provided does not match the expected type. Here’s the breakdown:\n\
                                    - Expected Type: String\n\
                                    - Received Type: {}\n\
                                ⚙ Suggested Action: Please ensure that the variable is a string. If you intended to provide a different type, here are some tips:\n\
                                    - For an integer, make sure it’s a whole number (e.g., 5).\n\
                                    - For a float, ensure it includes a decimal point (e.g., 3.14).\n\
                                    - If you need a string, it should be wrapped in quotes (e.g., \"example\").\n\
                                Let’s get the types sorted out to avoid this confusion!",
                                name,
                                match var_type {
                                    Vars::INT(_) => "integer",
                                    Vars::F(_) => "float",
                                    _ => "unknown type",
                                },
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
        "✘ Error: Variable Not Found\n\n\
        Oops! The variable '{}' could not be found in your code.\n\n\
        ➔ What Happened: This usually means that the variable was not declared, or it may be misspelled.\n\
        ➔ Suggested Action: Here are a few things you can check:\n\
            - Ensure that the variable is declared before you try to use it. For example:\n\
                ⚙ Example: 'may {} = value;'\n\
            - Check for any spelling errors. Even a small typo can lead to this error!\n\
            - Make sure that the variable is in the correct scope. If it’s declared inside a function, it won’t be accessible outside.\n\
        Let’s make sure that all your variables are properly declared and accessible!",
        vrname,vrname
    ))
}
