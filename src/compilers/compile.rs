use std::{
    fs,
    io::Write,
    process::{exit, Command},
};

pub fn compile_linux(asm: &String) {
    // Create the temporary ASM file
    match fs::File::create("./temp.asm") {
        Ok(mut asmf) => match asmf.write_all(asm.as_bytes()) {
            Ok(_) => {
                // Assemble the file
                let status = Command::new("nasm")
                    .args(["-f", "elf64"]) // Specify 64-bit ELF format
                    .arg("-o")
                    .arg("temp.o")
                    .arg("temp.asm")
                    .status()
                    .expect("Failed to execute `nasm` command");

                if !status.success() {
                    eprintln!("Error: Assembly failed");
                    exit(1);
                }

                // Link the object file
                let status = Command::new("ld")
                    .arg("-o")
                    .arg("output")
                    .arg("temp.o")
                    .status()
                    .expect("Failed to execute `ld` command");

                if !status.success() {
                    eprintln!("Error: Linking failed");
                    exit(1);
                }

                // Optionally, you can delete the temporary files after successful linking
                //fs::remove_file("temp.asm").expect("Failed to delete temporary ASM file");
                fs::remove_file("temp.o").expect("Failed to delete temporary object file");
            }
            Err(_) => {
                eprintln!("Error: Unable to write assembly code to file\nHint: Ensure correct permissions");
                exit(1);
            }
        },
        Err(_) => {
            eprintln!("Error: Unable to create assembly file\nHint: Ensure correct permissions");
            exit(1);
        }
    }
}
