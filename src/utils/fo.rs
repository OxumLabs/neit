use std::path::Path;
pub fn checkproj(pf: &String) -> Result<(), String> {
    let path = Path::new(&pf);

    if !path.exists() {
        return Err(format!("Path '{}' does not exist", pf));
    }

    if !path.is_dir() {
        return Err(format!("Path '{}' is not a directory", pf));
    }

    let main_nsc_path = path.join("main.nsc");

    if !main_nsc_path.exists() {
        return Err(format!("'main.nsc' file not found in '{}'", pf));
    }

    // Check if the 'main.nsc' is a file
    if !main_nsc_path.is_file() {
        return Err(format!("'main.nsc' exists but is not a file in '{}'", pf));
    }

    // If all checks pass, return Ok
    Ok(())
}
