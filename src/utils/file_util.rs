use crate::types::errors::CommonsError;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

// Returns the content of a file for a given path
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, CommonsError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Writes a string content into a file for a given path.
// Overwrites the file if it exists, creates a new one if it does not.
pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), CommonsError> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}
