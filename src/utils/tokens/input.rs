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
                            "🚫 Whoa there, buddy! You're trying to mess with the static variable '{}', but I can't let you do that! 😅\n\
                             🛑 Static variables are like rocks—once set, they don’t change!\n\
                             🤔 If you really wanna change it, maybe try making it mutable? Like this:\n\
                             \tlet mut {} = <value>\n\
                             (Just a suggestion, no pressure! 😎)",
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
                                "🤔 Hmm... I was expecting a string for '{}', but it looks like you've given me a {}.\n\
                                 🛠️ I was really hoping for: String.\n\
                                 ✏️ Could you double-check and make sure you're using the right type?",
                                name,
                                match var_type {
                                    Vars::INT(_) => "integer",
                                    Vars::F(_) => "float",
                                    _ => "something I don't recognize",
                                }
                            ))
                            
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Return an error if the variable was not found
    Err(format!(
        "❓ Uhhh... where did the variable '{}' go? 🤔\n\
         🔍 I looked everywhere, but I just can't find it!\n\
         ✏️ Maybe double-check if you spelled it right or declared it? My memory isn't the best sometimes! 😅",
        vrname
    ))
    
}
