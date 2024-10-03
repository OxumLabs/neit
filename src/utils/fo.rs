use std::path::Path;

pub fn checkproj(pf: &String) -> Result<(), String> {
    let path = Path::new(&pf);

    // Check if the project path exists
    if !path.exists() {
        return Err(format!(
            "🚫 Oopsie! I can’t find the specified path '{}'—it’s like a hidden treasure that doesn’t exist! 🗺️✨\n\
            🔍 Please double-check that the path is correct and give it another shot! Let’s find that treasure together! 🏴‍☠️",
            pf
        ));
    }

    // Check if the path is a directory
    if !path.is_dir() {
        return Err(format!(
            "🚫 Uh-oh! The specified path '{}' isn’t a directory—it's more like a mirage! 🌵✨\n\
            🔍 Please provide a valid project directory so we can get things rolling! Let’s find the right path together! 🗺️😊",
            pf
        ));
    }

    let main_nsc_path = path.join("main.nsc");

    // Check if the 'main.nsc' file exists
    if !main_nsc_path.exists() {
        return Err(format!(
            "🚫 Oh no! I can’t find the 'main.nsc' file in '{}'. It seems to have vanished! 🕵️‍♂️✨\n\
            🔍 Make sure the file is present in the directory—let’s not leave any important files behind! 🗂️😊",
            pf
        ));
    }

    // Check if 'main.nsc' is a file
    if !main_nsc_path.is_file() {
        return Err(format!(
            "🚫 Yikes! The 'main.nsc' file is hanging out at '{}' but it’s not a valid file. 😱\n\
            🔍 Please check the file's integrity—maybe it needs a little file-making to get back in shape! 🛠️✨\n\
            Let’s make sure everything is good to go! 😊",
            main_nsc_path.display()
        ));
    }

    // If all checks pass, return Ok
    Ok(())
}
