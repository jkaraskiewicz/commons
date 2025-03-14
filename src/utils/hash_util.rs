use super::file_util::read_file;
use crate::types::errors::CommonsError;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub fn get_file_hash(path: &PathBuf) -> Result<String, CommonsError> {
    let content = read_file(path)?;

    let mut hasher = Sha256::new();
    hasher.update(content);

    let hash = format!("{:x}", hasher.finalize());

    Ok(hash)
}
