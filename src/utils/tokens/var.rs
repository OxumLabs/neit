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
            "{}: Invalid keyword in line '{}'.\n\
            Reason: The line must start with '{}'.\n\
            Hint: Use 'may variable_name = value' for mutable or 'must variable_name = value' for immutable variables.",
            "✘ Error", trimmed_code, keyword
        ));
    }

    let pts: Vec<&str> = trimmed_code
        .trim_start_matches(keyword)
        .split('=')
        .collect();
    if pts.len() != 2 {
        return Err(format!(
            "{}: Invalid syntax in line '{}'.\n\
            Reason: The line must contain exactly one '=' sign separating the variable name and value.\n\
            Hint: Use the format '{}variable_name = value'.\n\
            - 'variable_name' should be a valid identifier (alphanumeric or underscores).\n\
            - 'value' should be any non-empty value you want to assign.\n\
            Example: '{}my_var = 42'",
            "✘ Error", trimmed_code, keyword, keyword
        ));
    }

    let (var_name, var_value) = (pts[0].trim(), pts[1].trim());

    // Validate variable name
    if var_name.is_empty() {
        return Err(format!(
            "{}: Empty variable name in line '{}'.\n\
            Reason: The variable name cannot be empty.\n\
            Hint: Provide a valid variable name before the '=' sign.\n\
            Example: '{}my_var = 42'",
            "✘ Error", trimmed_code, keyword
        ));
    }
    if CKWRDS.contains(&var_name) {
        return Err(format!(
            "{}: Invalid variable name '{}'.\n\
            Reason: The variable name cannot be a C keyword.\n\
            Hint: Try adding any letter to the variable or using a '_'.\n\
            Example: 'charr = 42'",
            "✘ Error", var_name
        ));
    }

    if !var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(format!(
            "{}: Invalid variable name '{}' in line '{}'.\n\
            Reason: The variable name can only contain alphanumeric characters or underscores.\n\
            Hint: Ensure the variable name follows naming conventions.\n\
            Example: '{}valid_name = 42'",
            "✘ Error", var_name, trimmed_code, keyword
        ));
    }
    if var_value.is_empty() {
        return Err(format!(
            "{}: Empty variable value in line '{}'.\n\
            Reason: The value after '=' cannot be empty.\n\
            Hint: Provide a valid value after the '=' sign.\n\
            Example: '{}my_var = 42'",
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
