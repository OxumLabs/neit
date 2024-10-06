use std::{fs::File, io::Read};

#[derive(Debug)] // This allows us to print the structure easily with `dbg!`
pub struct Grammar {
    pub def: String,
    pub new: String,
}

/// Process File with custom grammar and return the neit code
pub fn process_files(
    input_file: &str,
    user_grammar_file: Option<&str>,
    neit_file: Option<&str>,
) -> String {
    let mut nc = String::new();
    let defengine = gen_grm();
    let mut usrgrm: Vec<Grammar> = Vec::new();

    // Process the input grammar file
    if !input_file.is_empty() {
        process_grammar_file(input_file, &mut usrgrm);
    }

    // Process the user grammar file if provided
    if let Some(file) = user_grammar_file {
        process_grammar_file(file, &mut usrgrm);
    }

    // Process the neit file if provided
    if let Some(file) = neit_file {
        nc = process_neit_file(file, &usrgrm, &defengine);
    }
    nc
}

// Function to process grammar files
pub fn process_grammar_file(file_path: &str, usrgrm: &mut Vec<Grammar>) {
    let mut file = File::open(file_path).unwrap_or_else(|e| {
        eprintln!(
            "Could not find the grammar file '{}'. Ensure it exists: {}",
            file_path, e
        );
        std::process::exit(1);
    });

    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        eprintln!(
            "Error reading the source grammar file '{}': {}",
            file_path, e
        );
        std::process::exit(1);
    }

    let mut index = 1;
    for ln in content.lines() {
        if ln.starts_with('#') {
            continue; // Skip comments
        }

        let mut parts = ln.split('~');
        let ogv = parts.next().unwrap_or("").trim(); // Original value
        let nv = parts.next().unwrap_or("").trim(); // New value

        if parts.next().is_some() || ogv.is_empty() || nv.is_empty() {
            eprintln!(
                "Error on line({}) in the file '{}' : {}",
                index, file_path, ln
            );
            std::process::exit(1);
        }

        usrgrm.push(Grammar {
            def: ogv.to_string(),
            new: nv.to_string(),
        });
        index += 1;
    }
}

#[allow(unused)]
// Function to process the neit file
pub fn process_neit_file(file_path: &str, usrgrm: &[Grammar], defengine: &[Grammar]) -> String {
    let mut nc = String::new(); // This will store the final processed content
    let mut file = File::open(file_path).unwrap_or_else(|_| {
        eprintln!("Could not open neit file '{}'", file_path);
        std::process::exit(1);
    });

    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        eprintln!("Error reading file '{}': {}", file_path, e);
        std::process::exit(1);
    }

    // Store all processed lines here
    let mut processed_lines = Vec::new();

    // Split the content by lines
    for line in content.lines() {
        let mut modified_line = String::new();
        let mut current_word = String::new();
        let mut in_string_mode = false;

        // Now process each character in the current line
        for c in line.chars() {
            if c == '"' {
                in_string_mode = !in_string_mode;
                modified_line.push(c);
                continue;
            }

            if in_string_mode {
                modified_line.push(c);
            } else {
                if c.is_whitespace() || c.is_ascii_punctuation() {
                    if !current_word.is_empty() {
                        let replaced_word = replace_word(&current_word, usrgrm, defengine);
                        modified_line.push_str(&replaced_word);
                        current_word.clear();
                    }
                    modified_line.push(c);
                } else {
                    current_word.push(c);
                }
            }
        }

        // Append any remaining word after processing the line
        if !current_word.is_empty() {
            let replaced_word = replace_word(&current_word, usrgrm, defengine);
            modified_line.push_str(&replaced_word);
        }

        // Store the processed line
        processed_lines.push(modified_line);
    }

    // Join all the processed lines with new line characters
    nc = processed_lines.join("\n");
    nc
}

// Helper function to replace a word if it matches grammar definitions
fn replace_word(word: &str, usrgrm: &[Grammar], defengine: &[Grammar]) -> String {
    for mapping in usrgrm.iter().chain(defengine.iter()) {
        if word == mapping.new {
            return mapping.def.clone();
        }
    }
    word.to_string()
}

// Function to generate default grammar mappings
pub fn gen_grm() -> Vec<Grammar> {
    vec![
        Grammar {
            def: "fn".to_string(),
            new: "fn".to_string(),
        },
        Grammar {
            def: "may".to_string(),
            new: "may".to_string(),
        },
        Grammar {
            def: "must".to_string(),
            new: "must".to_string(),
        },
        Grammar {
            def: "=".to_string(),
            new: "=".to_string(),
        },
        Grammar {
            def: "pub".to_string(),
            new: "pub".to_string(),
        },
        Grammar {
            def: "takein".to_string(),
            new: "takein".to_string(),
        },
        Grammar {
            def: "println".to_string(),
            new: "println".to_string(),
        },
        Grammar {
            def: "print".to_string(),
            new: "print".to_string(),
        },
        Grammar {
            def: "=".to_string(),
            new: "=".to_string(),
        },
    ]
}
