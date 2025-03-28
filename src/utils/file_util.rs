use crate::types::errors::CommonsError;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, CommonsError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

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

pub fn exists_directory<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

pub fn exists_file<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}
