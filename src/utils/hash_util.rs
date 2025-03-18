use super::file_util::read_file;
use crate::types::errors::CommonsError;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub fn get_string_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);

    format!("{:x}", hasher.finalize())
}

pub fn get_file_hash(path: &PathBuf) -> Result<String, CommonsError> {
    let content = read_file(path)?;
    Ok(get_string_hash(&content))
}
