use std::path::Path;

pub fn checkproj(pf: &String) -> Result<(), String> {
    let path = Path::new(&pf);

    // Check if the project path exists
    if !path.exists() {
        return Err(format!(
            "Error: The specified path '{}' does not exist.\nPlease ensure the path is correct and try again.",
            pf
        ));
    }

    // Check if the path is a directory
    if !path.is_dir() {
        return Err(format!(
            "Error: The specified path '{}' is not a directory.\nPlease provide a valid project directory.",
            pf
        ));
    }

    let main_nsc_path = path.join("main.nsc");

    // Check if the 'main.nsc' file exists
    if !main_nsc_path.exists() {
        return Err(format!(
            "Error: The 'main.nsc' file was not found in '{}'.\nMake sure the file is present in the directory.",
            pf
        ));
    }

    // Check if 'main.nsc' is a file
    if !main_nsc_path.is_file() {
        return Err(format!(
            "Error: The 'main.nsc' file exists at '{}' but is not a valid file.\nPlease check the file's integrity.",
            main_nsc_path.display()
        ));
    }

    // If all checks pass, return Ok
    Ok(())
}
