use crate::types::errors::CommonsError;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

/// Returns the content of a file for a given path as UTF-8 string
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, CommonsError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Returns the content of a file for a given path as Vec<u8>
pub fn read_binary_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, CommonsError> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Writes a string content into a file for a given path.
/// Overwrites the file if it exists, creates a new one if it does not.
pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), CommonsError> {
    let count = path.as_ref().components().count();
    let dir_path: PathBuf = path.as_ref().components().take(count - 1).collect();
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path)?;
    }

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

/// Writes a binary content into a file for a given path.
/// Overwrites the file if it exists, creates a new one if it does not.
pub fn write_binary_file<P: AsRef<Path>>(path: P, content: &[u8]) -> Result<(), CommonsError> {
    let count = path.as_ref().components().count();
    let dir_path: PathBuf = path.as_ref().components().take(count - 1).collect();
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path)?;
    }

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    file.write_all(content)?;
    file.flush()?;
    Ok(())
}

/// Removes a file/directory at a given path
pub fn delete_file<P: AsRef<Path>>(path: P) -> Result<(), CommonsError> {
    if path.as_ref().is_file() {
        std::fs::remove_file(path)?;
    } else {
        std::fs::remove_dir_all(path)?;
    };
    Ok(())
}

/// Copies a file or a directory
pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), CommonsError> {
    let source_path = from.as_ref();
    let destination_path = to.as_ref();

    if source_path.is_dir() {
        dircpy::copy_dir(source_path, destination_path)?;
    } else {
        let count = destination_path.components().count();
        let dir_path: PathBuf = destination_path.components().take(count - 1).collect();
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
        }
        fs::copy(source_path, destination_path)?;
    };

    Ok(())
}
