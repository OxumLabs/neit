use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub fn pbc(bc: &String) {
    let executable_path = "./nrlin";
    println!("[DEBUG] Executable path set to: {}", executable_path);

    // Open the existing executable
    println!("[DEBUG] Attempting to open the executable file.");
    let mut original_file = match File::open(executable_path) {
        Ok(file) => {
            println!("[DEBUG] Successfully opened file: {}", executable_path);
            file
        }
        Err(e) => {
            eprintln!("[ERROR] Error opening file {}: {}", executable_path, e);
            return;
        }
    };

    // Read the contents of the original executable
    println!("[DEBUG] Reading the contents of the original executable.");
    let mut contents = Vec::new();
    if let Err(e) = original_file.read_to_end(&mut contents) {
        eprintln!("[ERROR] Error reading file {}: {}", executable_path, e);
        return;
    }
    println!("[DEBUG] Read {} bytes from the original file.", contents.len());

    // Get the original size
    let original_size = contents.len() as u64;
    println!("[DEBUG] Original file size: {} bytes", original_size);

    // Create the final output file
    println!("[DEBUG] Creating the output file 'fnrlin'.");
    let mut output_file = match OpenOptions::new().write(true).create(true).open("fnrlin") {
        Ok(file) => {
            println!("[DEBUG] Successfully created output file: fnrlin");
            file
        }
        Err(e) => {
            eprintln!("[ERROR] Error creating output file: {}", e);
            return;
        }
    };

    // Write original contents
    println!("[DEBUG] Writing original file contents to 'fnrlin'.");
    if let Err(e) = output_file.write_all(&contents) {
        eprintln!("[ERROR] Error writing original contents: {}", e);
        return;
    }
    println!("[DEBUG] Successfully wrote original contents.");

    // Append data from the provided string
    println!("[DEBUG] Appending additional data to the output file.");
    if let Err(e) = output_file.write_all(bc.as_bytes()) {
        eprintln!("[ERROR] Error writing additional data: {}", e);
        return;
    }
    println!("[DEBUG] Successfully appended additional data.");

    // Append original size as the last 10 bytes
    println!("[DEBUG] Appending original file size as the last 10 bytes.");
    let mut size_bytes = original_size.to_le_bytes().to_vec();
    size_bytes.resize(10, 0); // Ensure it occupies exactly 10 bytes
    if let Err(e) = output_file.write_all(&size_bytes) {
        eprintln!("[ERROR] Error writing original size: {}", e);
        return;
    }
    println!("[DEBUG] Successfully appended original size.");

    println!("[DEBUG] File successfully written to 'fnrlin'.");
}
