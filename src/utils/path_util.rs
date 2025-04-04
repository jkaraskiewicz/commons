use crate::types::errors::CommonsError;
use std::env;
use std::path::PathBuf;

/// Takes a path passed as a string and replaces the occurences
/// of ~ with the HOME directory.
pub fn expand_home_dir_in_path(path: &str) -> Result<PathBuf, CommonsError> {
    let home_dir = env::var("HOME")?;
    if path.starts_with("~") {
        let expanded_path = path.replacen("~", &home_dir, 1);
        Ok(PathBuf::from(expanded_path))
    } else {
        Ok(PathBuf::from(path))
    }
}

/// Returns a path to the HOME directory.
pub fn get_home_dir_path() -> Result<PathBuf, CommonsError> {
    let home_dir = env::var("HOME")?;
    Ok(PathBuf::from(home_dir))
}
