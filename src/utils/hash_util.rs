use super::file_util::read_file;
use crate::types::errors::CommonsError;
use sha2::{Digest, Sha256};
use std::path::Path;

// Returns a SHA256 hash for a given input string
pub fn get_string_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

// Returns a SHA256 hash of the contents of a file given by path
pub fn get_file_hash<P: AsRef<Path>>(path: P) -> Result<String, CommonsError> {
    let content = read_file(path.as_ref())?;
    Ok(get_string_hash(&content))
}
//
// Generates a UID for a given string input by calculating
// a SHA256 hash and truncating it.
pub fn generate_uid(input: &str) -> String {
    let mut result = get_string_hash(input);
    result.truncate(8);
    result
}
