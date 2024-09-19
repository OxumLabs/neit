use crate::utils::types::Vars;

pub fn process_var(code: &str) -> Result<(Vars, String), String> {
    let pts: Vec<&str> = code.trim_start_matches("may ").split('=').collect();
    if pts.len() != 2 {
        return Err(format!(
            "✘ Error: Invalid syntax in line '{}'.\n\
            Reason: The line must contain exactly one '=' sign separating the variable name and value.\n\
            Hint: Use the format 'may variable_name = value'.\n\
            - 'variable_name' should be a valid identifier (alphanumeric or underscores).\n\
            - 'value' should be any non-empty value you want to assign.\n\
            Example: 'may my_var = 42'",
            code.trim()
        ));
    }
    let (var_name, var_value) = (pts[0].trim(), pts[1].trim());
    if var_name.is_empty() {
        return Err(format!(
            "✘ Error: Empty variable name in line '{}'.\n\
            Reason: The variable name cannot be empty.\n\
            Hint: Provide a valid variable name before the '=' sign.\n\
            Example: 'may my_var = 42'",
            code.trim()
        ));
    }
    if !var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(format!(
            "✘ Error: Invalid variable name '{}' in line '{}'.\n\
            Reason: The variable name can only contain alphanumeric characters or underscores.\n\
            Hint: Ensure the variable name follows naming conventions.\n\
            Example: 'may valid_name = 42'",
            var_name,
            code.trim()
        ));
    }
    if var_value.is_empty() {
        return Err(format!(
            "✘ Error: Empty variable value in line '{}'.\n\
            Reason: The value after '=' cannot be empty.\n\
            Hint: Provide a valid value after the '=' sign.\n\
            Example: 'may my_var = 42'",
            code.trim()
        ));
    }

    let mut vr = Vars::new();
    match vr.update_type(var_value) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }
    //println!("Var : {:?}", vr);

    Ok((vr, var_name.to_string()))
}
