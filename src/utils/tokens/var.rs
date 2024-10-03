use crate::utils::types::{Tokens, Vars};

static CKWRDS: &[&str] = &[
    "auto",
    "break",
    "case",
    "char",
    "const",
    "continue",
    "default",
    "do",
    "double",
    "else",
    "enum",
    "extern",
    "float",
    "for",
    "goto",
    "if",
    "int",
    "long",
    "register",
    "return",
    "short",
    "signed",
    "sizeof",
    "static",
    "struct",
    "switch",
    "typedef",
    "union",
    "unsigned",
    "void",
    "volatile",
    "while",
    "_Alignas",
    "_Alignof",
    "_Atomic",
    "_Bool",
    "_Complex",
    "_Generic",
    "_Imaginary",
    "_Noreturn",
    "_Static_assert",
    "_Thread_local",
    "inline",
    "restrict",
    "_Pragma",
];

pub fn process_var(code: &str, vrs: &Vec<Tokens>, whole: bool) -> Result<(Vars, String), String> {
    // Trim leading whitespace and determine the keyword used
    let trimmed_code = code.trim();
    let keyword = if whole { "may " } else { "must " };

    // Check if the code starts with the correct keyword
    if !trimmed_code.starts_with(keyword) {
        return Err(format!(
            "{} Oopsie! I found an invalid keyword in line '{}'. 😅\n\
            🚫 Reason: The line should start with '{}'.\n\
            🔍 Hint: Remember to use 'may variable_name = value' for mutable variables or 'must variable_name = value' for immutable ones!\n\
            Let’s keep it tidy! 🎉",
            "✘ Error", trimmed_code, keyword
        ));
    }

    let pts: Vec<&str> = trimmed_code
        .trim_start_matches(keyword)
        .split('=')
        .collect();
    if pts.len() != 2 {
        return Err(format!(
            "{} Uh-oh! I found some invalid syntax in line '{}'. 😬\n\
            🚫 Reason: There should be exactly one '=' sign separating the variable name and value!\n\
            🔍 Hint: Use the format '{}variable_name = value'.\n\
            - Remember, 'variable_name' should be a valid identifier (think alphanumeric or underscores).\n\
            - And 'value' needs to be something non-empty that you want to assign!\n\
            🎉 Example: '{}my_var = 42'\n\
            Let's get it right! 😊",
            "✘ Error", trimmed_code, keyword, keyword
        ));
    }

    let (var_name, var_value) = (pts[0].trim(), pts[1].trim());

    // Validate variable name
    if var_name.is_empty() {
        return Err(format!(
            "{} Whoopsie! I spotted an empty variable name in line '{}'. 😅\n\
            🚫 Reason: The variable name can’t be empty—let’s give it a proper name!\n\
            🔍 Hint: Make sure to provide a valid variable name before the '=' sign.\n\
            🎉 Example: '{}my_var = 42'\n\
            Let’s tidy this up! 😊",
            "✘ Error", trimmed_code, keyword
        ));
    }
    if CKWRDS.contains(&var_name) {
        return Err(format!(
            "{} Oh no! I see an invalid variable name '{}'. 🤦‍♂️\n\
            🚫 Reason: You can't use a C keyword as a variable name—those words have special powers! ✨\n\
            🔍 Hint: Try spicing it up by adding a letter or tossing in an underscore ('_').\n\
            🎉 Example: Instead of using a keyword, how about naming it 'charr = 42'? It’s quirky and fun!\n\
            🧐 Remember, variables need unique names to shine—let's give your variable a proper identity! 🎈",
            "✘ Error", var_name
        ));
    }

    if !var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(format!(
            "{} Uh-oh! I found an invalid variable name '{}' in line '{}'. 😬\n\
            🚫 Reason: Your variable name can only fill itself with alphanumeric characters or underscores—no funny business allowed! 🛑\n\
            🔍 Hint: Let's make sure it follows the naming conventions! Keep it classy, folks! ✨\n\
            🎉 Example: You could use something like '{}valid_name = 42'. Simple and elegant, just like a fine cheese! 🧀\n\
            Remember, names matter—let's give your variable a fabulous identity! 🎈",
            "✘ Error", var_name, trimmed_code, keyword
        ));
    }
    if var_value.is_empty() {
        return Err(format!(
            "{} Oops! I’ve spotted an empty variable value in line '{}'. 😅\n\
            🚫 Reason: The value after '=' can’t be empty—let's fill it in! ✨\n\
            🔍 Hint: Make sure to provide a valid value after the '=' sign.\n\
            🎉 Example: You could write something like '{}my_var = 42'. It’s nice and straightforward! 😊\n\
            Remember, every variable deserves a value—let’s make it happen! 🎈",
            "✘ Error", trimmed_code, keyword
        ));
    }

    let mut vr = Vars::new();
    match vr.update_type(var_value, vrs) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    Ok((vr, var_name.to_string()))
}
