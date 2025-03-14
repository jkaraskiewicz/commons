use std::{env, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonsError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Var error: {0}")]
    VarError(#[from] env::VarError),
}
