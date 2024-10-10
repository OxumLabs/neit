use crate::utils::types::{Tokens, Vars};

// Static array of C keywords to avoid using them as variable names
pub static C_KEYWORDS: &[&str] = &[
    "alignas",
    "alignof",
    "and",
    "and_eq",
    "asm",
    "auto",
    "bitand",
    "bitor",
    "bool",
    "break",
    "case",
    "catch",
    "char",
    "char16_t",
    "char32_t",
    "class",
    "compl",
    "const",
    "constexpr",
    "const_cast",
    "continue",
    "decltype",
    "default",
    "delete",
    "do",
    "double",
    "dynamic_cast",
    "else",
    "enum",
    "explicit",
    "export",
    "extern",
    "false",
    "float",
    "for",
    "friend",
    "goto",
    "if",
    "inline",
    "int",
    "long",
    "mutable",
    "namespace",
    "new",
    "noexcept",
    "not",
    "not_eq",
    "nullptr",
    "operator",
    "or",
    "or_eq",
    "private",
    "protected",
    "public",
    "register",
    "reinterpret_cast",
    "return",
    "short",
    "signed",
    "sizeof",
    "static",
    "static_assert",
    "static_cast",
    "struct",
    "switch",
    "synchronized",
    "template",
    "this",
    "throw",
    "true",
    "try",
    "typedef",
    "typeid",
    "typename",
    "union",
    "unsigned",
    "using",
    "virtual",
    "void",
    "volatile",
    "wchar_t",
    "while",
    "xor",
    "xor_eq",
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
            "✘ Error: Invalid Variable Name\n\
            Oh no! I see an invalid variable name: '{}'.\n\
            ➔ Issue: This name is a reserved C keyword and cannot be used.\n\
            ➔ Suggested Action: Change the variable name to avoid keywords. Consider adding a letter, number, or underscore ('_') for uniqueness.\n\
            ➔ Example: Instead of using 'char', use 'char_variable = 42'.\n\
            Let's ensure our variable names are unique and valid!",
            var_name
        ));
    }

    // Validate variable name contains only valid characters
    if !var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(format!(
            "✘ Error: Invalid Variable Name Found\n\
            Invalid variable name '{}' at line '{}'.\n\
            ➔ Reason: Names can only use alphanumeric characters or underscores—no special characters!\n\
            ➔ Hint: Follow naming conventions for clarity.\n\
            ➔ Example: Use something like '{}valid_name = 42'.\n\
            Let's give your variable a valid identity!",
            var_name, trimmed_code, expected_keyword
        ));
    }

    // Validate variable value is not empty
    if var_value.is_empty() {
        return Err(format!(
            "✘ Error: Empty Variable Value Found\n\
            Empty variable value detected at line '{}'.\n\
            ➔ Reason: The assignment is missing a value after the '=' sign!\n\
            ➔ Suggested Action: Provide a valid value for the variable.\n\
            ➔ Example: Instead of leaving it empty, use '{}my_var = 42'.\n\
            Every variable deserves a value—let's fix this to keep your code running smoothly!",
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
