use crate::utils::types::{Args, FN};

#[allow(unused)]
pub fn process_pub_func(ln: &str, index: usize) -> Result<FN, String> {
    // Initialize an empty FN struct
    let mut function = FN::new(String::new(), false, Vec::new(), Vec::new());
    let ln = ln.trim();
    if ln.trim().starts_with("pub fn") {
        let parts: Vec<&str> = ln[7..].split('(').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Error at line {}: Invalid Function Declaration.\n\nHint: Ensure the function is declared correctly. Example: \n    pub fn function_name(args: Type) {{}}\n\nCheck for misplaced parentheses or missing argument list.\n\nDeclaration found: `{}`",
                index, ln
            ));
        }

        // Set the function name
        function.name = parts[0].trim().to_owned();
        if function.name.is_empty() {
            return Err(format!(
                "Error at line {}: Function name is missing.\n\nHint: Add a valid function name after `pub fn`. Example:\n    pub fn my_function() {{}}\n\nDeclaration found: `{}`",
                index, ln
            ));
        }

        // Handle function arguments
        let args_str = parts[1].trim_end_matches("){}").trim();
        if !args_str.is_empty() {
            let farg = args_str.split(',');
            for i in farg {
                let mut pts = i.split(':');

                let nm = pts
                .next()
                .ok_or_else(|| format!("Error at line {}: Missing argument name.\n\nHint: Each argument should be in the format `name: Type`. Example:\n    pub fn my_function(x: i32) {{}}\n\nDeclaration found: `{}`", index, ln))?;

                let t = pts.next().ok_or_else(|| {
                format!(
                    "Error at line {}: Missing type for argument `{}`.\n      =>Hint: Each argument should have a type. Example:\n    pub fn my_function(x: i32) {{}}\nDeclaration found: `{}`",
                    index, nm,ln
                )
            })?;

                // Only allow known types
                match t.trim() {
                    "int" | "string" | "float" => {
                        function
                            .args
                            .push(Args::new(nm.trim().to_owned(), t.trim()));
                    }
                    _ => {
                        return Err(format!(
                        "Error at line {}: Unknown type `{}` for argument `{}`.\nHint: Ensure the argument type is a valid type, such as `int`, `String`, `bool`, etc.\nDeclaration found: `{}`",
                        index, t.trim(), nm.trim(), ln
                    ));
                    }
                }

                // Check for empty argument name
                if nm.trim().is_empty() {
                    return Err(format!(
                    "Error at line {}: Argument name is empty.\nHint: Ensure each argument has a valid name. Example:\n    pub fn my_function(x: i32) {{}}\n\nDeclaration found: `{}`",
                    index, ln
                ));
                }
            }
        } else {
            function.args.push(Args::new("_".to_string(), "emp"));
        }

        // Mark the function as global (public)
        function.is_global = true;

        // Return the constructed FN object
        return Ok(function);
    } else if ln.starts_with("fn ") {
        let parts: Vec<&str> = ln[3..].split('(').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Error at line {}: Invalid Function Declaration.\n\nHint: Ensure the function is declared correctly. Example: \n    pub fn function_name(args: Type) {{}}\n\nCheck for misplaced parentheses or missing argument list.\n\nDeclaration found: `{}`",
                index, ln
            ));
        }

        // Set the function name
        function.name = parts[0].trim().to_owned();
        if function.name.is_empty() {
            return Err(format!(
                "Error at line {}: Function name is missing.\n\nHint: Add a valid function name after `pub fn`. Example:\n    pub fn my_function() {{}}\n\nDeclaration found: `{}`",
                index, ln
            ));
        }

        // Handle function arguments
        let args_str = parts[1].trim_end_matches("){}").trim();
        if !args_str.is_empty() {
            let farg = args_str.split(',');
            for i in farg {
                let mut pts = i.split(':');

                let nm = pts
                .next()
                .ok_or_else(|| format!("Error at line {}: Missing argument name.\n\nHint: Each argument should be in the format `name: Type`. Example:\n    pub fn my_function(x: i32) {{}}\n\nDeclaration found: `{}`", index, ln))?;

                let t = pts.next().ok_or_else(|| {
                format!(
                    "Error at line {}: Missing type for argument `{}`.\n      =>Hint: Each argument should have a type. Example:\n    pub fn my_function(x: i32) {{}}\nDeclaration found: `{}`",
                    index, nm,ln
                )
            })?;

                // Only allow known types
                match t.trim() {
                    "int" | "string" | "float" => {
                        function
                            .args
                            .push(Args::new(nm.trim().to_owned(), t.trim()));
                    }
                    _ => {
                        return Err(format!(
                        "Error at line {}: Unknown type `{}` for argument `{}`.\nHint: Ensure the argument type is a valid type, such as `int`, `String`, `bool`, etc.\nDeclaration found: `{}`",
                        index, t.trim(), nm.trim(), ln
                    ));
                    }
                }

                // Check for empty argument name
                if nm.trim().is_empty() {
                    return Err(format!(
                    "Error at line {}: Argument name is empty.\nHint: Ensure each argument has a valid name. Example:\n    pub fn my_function(x: i32) {{}}\n\nDeclaration found: `{}`",
                    index, ln
                ));
                }
            }
        } else {
            function.args.push(Args::new("_".to_string(), "emp"));
        }

        // Mark the function as global (public)
        function.is_global = false;

        // Return the constructed FN object
        return Ok(function);
    } else {
        return Err(format!(
            "Error at line {}: Not a valid function declaration.\n\nHint: Function declarations must start with `pub fn`. Example:\n    pub fn my_function() {{}}\n\nDeclaration found: `{}`",
            index, ln
        ));
    }
}
