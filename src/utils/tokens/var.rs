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
    if !trimmed_code.starts_with(expected_keyword) {
        return Err(format!(
            "✘ Oops! I found an invalid keyword in line '{}'.\n\
            → Reason: The line should start with '{}'.\n\
            →→ Hint: Remember to use 'may variable_name = value' for mutable variables or 'must variable_name = value' for immutable ones!\n\
                     Let’s keep it tidy!",
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
            "✘ Uh-oh! I found some invalid syntax in line '{}'.\n\
            → Reason: There should be exactly one '=' sign separating the variable name and value!\n\
            →→ Hint: Use the format '{}variable_name = value'.\n\
            - Remember, 'variable_name' should be a valid identifier (think alphanumeric or underscores).\n\
            - And 'value' needs to be something non-empty that you want to assign!\n\
            Example: '{}my_var = 42'\n\
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
            "✘ Oh no! I see an invalid variable name '{}'.\n\
            → Reason: You can't use a C keyword as a variable name—those words have special meanings!\n\
            →→ Hint: Try modifying it by adding a letter or an underscore ('_').\n\
                     Example: Instead of using a keyword, how about naming it 'char_variable = 42'?",
            var_name
        ));
    }

    // Validate variable name contains only valid characters
    if !var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(format!(
            "✘ Uh-oh! I found an invalid variable name '{}' in line '{}'.\n\
            → Reason: Your variable name can only consist of alphanumeric characters or underscores—no funny business allowed!\n\
            →→ Hint: Let's make sure it follows the naming conventions! Keep it classy!\n\
                    Example: You could use something like '{}valid_name = 42'. Simple and elegant!\n\
                    Remember, names matter—let's give your variable a fabulous identity!",
            var_name, trimmed_code, expected_keyword
        ));
    }

    // Validate variable value is not empty
    if var_value.is_empty() {
        return Err(format!(
            "✘ Oops! I’ve spotted an empty variable value in line '{}'.
            → Reason: The value after '=' can’t be empty—let's fill it in!\n\
            →→ Hint: Make sure to provide a valid value after the '=' sign.\n\
                     Example: You could write something like '{}my_var = 42'. It’s nice and straightforward!\n\
                     Remember, every variable deserves a value—let’s make it happen!",
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
