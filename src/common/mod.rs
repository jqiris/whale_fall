pub mod file;
pub mod go;
pub mod str;
use anyhow::{anyhow, Error, Result};
use handlebars::Handlebars;
use serde::Serialize;

pub fn render_template<T>(reg: Handlebars, template_string: &str, data: &T) -> Result<String>
where
    T: Serialize,
{
    match reg.render_template(template_string, data) {
        Ok(output) => Ok(output),
        Err(err) => Err(anyhow!(err)),
    }
}

pub fn option_to_result<T>(option: Option<T>, err: Error) -> Result<T> {
    option.ok_or(err)
}
