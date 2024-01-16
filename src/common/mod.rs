pub mod file;
pub mod str;
#[cfg(test)]
mod tests;
use anyhow::{Error, Result};

pub fn option_to_result<T>(option: Option<T>, err: Error) -> Result<T> {
    option.ok_or(err)
}
