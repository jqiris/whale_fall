pub mod file;
pub mod go;
pub mod str;
use anyhow::{Error, Result};

pub fn option_to_result<T>(option: Option<T>, err: Error) -> Result<T> {
    option.ok_or(err)
}
