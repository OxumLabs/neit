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
                        println!("var : {:?}", token);
                        return Err(format!(
                            "🚫 Oops! You can't mutate the static variable '{}'.\n\
                             🔧 To fix this, try declaring it as mutable like this:\n\
                             \tmay {} = <value>",
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
                                "⚠️ Uh-oh! The variable '{}' is not a string.\n\
                                 🔍 Expected: String\n\
                                 🛠️ Actual: {}\n\
                                 ✏️ Make sure you're using the right type in your input.",
                                name,
                                format!(
                                    "{}",
                                    match var_type {
                                        Vars::INT(_) => "integer",
                                        Vars::F(_) => "float",
                                        _ => "undefined!?",
                                    }
                                )
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
        "❓ Hmmm... I couldn't find the variable '{}'.\n\
         🔍 Double-check that it's declared and there's no typo.\n\
         ✏️ Make sure it's defined before you try to use it.",
        vrname
    ))
}
