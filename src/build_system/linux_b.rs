use std::fs::File;
use std::io::Write;
use std::process::Command;

use crate::nulibc::{self, NULIBCH};

pub fn linux_b_64(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Write the source to a temporary C file
    let c_file = "temp.c";
    let mut file = File::create(c_file)?;
    file.write_all(source.as_bytes())?;

    // Write nulibc header
    let nulibc_h = "nulibc.h";
    let mut header_file = File::create(nulibc_h)?;
    header_file.write_all(NULIBCH.as_bytes())?;

    // Write nulibc implementation
    let nulibc_c = "nulibc.c";
    let mut impl_file = File::create(nulibc_c)?;
    impl_file.write_all(nulibc::NULIBC.as_bytes())?;

    // Compile with GCC statically
    let output = Command::new("gcc")
        .args(&["-static", "temp.c", "nulibc.c", "-o", "output"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Compilation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // Clean up temporary files
    std::fs::remove_file(c_file)?;
    std::fs::remove_file(nulibc_h)?;
    std::fs::remove_file(nulibc_c)?;

    Ok(())
}
