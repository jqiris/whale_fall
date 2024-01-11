use anyhow::{anyhow, Result};
use handlebars::{handlebars_helper, Handlebars};
use lazy_static::lazy_static;
use serde::Serialize;
use std::sync::Mutex;

use crate::tpls::miman::type_def::Field;

handlebars_helper!(field_access: |x: Field, y: String|  format!("{}.{}", y, x.field));

lazy_static! {
    static ref TPLS: Mutex<Handlebars<'static>> = {
        let mut reg = Handlebars::new();
        reg.register_helper("field_access", Box::new(field_access));
        Mutex::new(reg)
    };
}

pub fn render_template<T>(template_string: &str, data: &T) -> Result<String>
where
    T: Serialize,
{
    match TPLS.lock().unwrap().render_template(template_string, data) {
        Ok(output) => Ok(output),
        Err(err) => Err(anyhow!(err)),
    }
}
