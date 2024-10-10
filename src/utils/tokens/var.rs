use crate::utils::types::{Tokens, Vars};

// Static array of C keywords to avoid using them as variable names
static C_KEYWORDS: &[&str] = &[
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

pub fn process_var(
    code: &str,
    variables: &Vec<Tokens>,
    is_whole: bool,
) -> Result<(Vars, String), String> {
    // Trim leading whitespace and determine the expected keyword based on context
    let trimmed_code = code.trim();
    let expected_keyword = if is_whole { "may " } else { "must " };

    // Check if the line starts with the expected keyword
    //println!("key word : {}", expected_keyword);
    if !trimmed_code.starts_with(expected_keyword) {
        return Err(format!(
            "✘ Error: Invalid Keyword Found\n\n\
            Oops! I found an invalid keyword in line '{}'.\n\n\
            ➔ Reason: The line should start with '{}'.\n\
            ➔ Hint: Remember to use:\n\
                1. 'may variable_name = value' for mutable variables.\n\
                2. 'must variable_name = value' for immutable variables.\n\n\
            → Let’s keep it tidy and ensure all lines follow the correct syntax!",
            trimmed_code, expected_keyword
        ));
    }

    // Split the trimmed code into variable name and value parts
    let parts: Vec<&str> = trimmed_code
        .trim_start_matches(expected_keyword)
        .split('=')
        .collect();

    // Ensure there's exactly one '=' sign separating name and value
    if parts.len() != 2 {
        return Err(format!(
            "✘ Error: Invalid Syntax Found\n\n\
            Uh-oh! I found some invalid syntax in line '{}'.\n\n\
            ➔ Reason: There should be exactly one '=' sign separating the variable name and value!\n\
            ➔ Hint: Use the format '{}variable_name = value'.\n\n\
            - Remember, 'variable_name' should be a valid identifier (think alphanumeric or underscores).\n\
            - The 'value' needs to be something non-empty that you want to assign!\n\n\
            ➔ Example: '{}my_var = 42'\n\n\
            Let's get it right!",
            trimmed_code, expected_keyword, expected_keyword
        ));
    }

    let (var_name, var_value) = (parts[0].trim(), parts[1].trim());

    // Validate variable name is not empty
    if var_name.is_empty() {
        return Err(format!(
            "✘ Whoops! I spotted an empty variable name in line '{}'.\n\
            → Reason: The variable name can’t be empty—let’s give it a proper name!\n\
            →→ Hint: Make sure to provide a valid variable name before the '=' sign.\n\
                     Example: '{}my_var = 42'\n\
                     Let’s tidy this up!",
            trimmed_code, expected_keyword
        ));
    }

    // Validate variable name is not a C keyword
    if C_KEYWORDS.contains(&var_name) {
        return Err(format!(
            "✘ Error: Invalid Variable Name\n\n\
            Oh no! I see an invalid variable name: '{}'.\n\n\
            ➔ What Happened: The name you chose for your variable is a reserved C keyword. These keywords have special meanings in the language and cannot be used as variable names.\n\
            ➔ Suggested Action: To fix this, try modifying the variable name to avoid using any C keywords.\n\
            ➔ Hint: A good practice is to add a letter, number, or underscore ('_') to make it unique.\n\n\
            ➔ Example: Instead of using a keyword like 'char', consider naming it 'char_variable = 42'.\n\n\
            Let's ensure our variable names are unique and valid to keep your code error-free!"
        , var_name
        ));
    }

    // Validate variable name contains only valid characters
    if !var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(format!(
            "✘ Error: Invalid Variable Name Found\n\n\
            Uh-oh! I found an invalid variable name '{}' in line '{}'.\n\n\
            ➔ Reason: Your variable name can only consist of alphanumeric characters or underscores—no funny business allowed!\n\
            ➔ Hint: Let's ensure it follows the naming conventions! Keep it classy!\n\n\
            ➔ Example: You could use something like '{}valid_name = 42'. Simple and elegant!\n\n\
            Remember, names matter—let's give your variable a fabulous identity!",
            var_name, trimmed_code, expected_keyword
        ));
    }

    // Validate variable value is not empty
    if var_value.is_empty() {
        return Err(format!(
            "✘ Error: Empty Variable Value Found\n\n\
            Oops! I’ve spotted an empty variable value in line '{}'.\n\n\
            ➔ What Happened: The variable assignment is missing a value! This means there’s nothing after the '=' sign.\n\
            ➔ Suggested Action: You need to provide a valid value for the variable so that it can be used in your code.\n\
            ➔ Hint: Make sure to provide a valid value right after the '=' sign.\n\n\
            ➔ Example: Instead of leaving it empty, try something like '{}my_var = 42'. It’s simple and effective!\n\n\
            Remember, every variable deserves a value—let’s make it happen and keep your code running smoothly!",
            trimmed_code, expected_keyword
        ));
    }

    // Create a new Vars instance and update its type based on the value provided
    let mut vr = Vars::new();
    match vr.update_type(var_value, variables) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    Ok((vr, var_name.to_string()))
}
