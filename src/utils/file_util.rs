use crate::types::errors::CommonsError;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

pub fn read_file(path: &PathBuf) -> Result<String, CommonsError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(path: &PathBuf, content: &str) -> Result<(), CommonsError> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn exists_directory(path: &PathBuf) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

pub fn exists_file(path: &PathBuf) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}
