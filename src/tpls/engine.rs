use anyhow::{anyhow, Result};
use handlebars::{handlebars_helper, Handlebars};
use lazy_static::lazy_static;
use serde::Serialize;
use std::sync::Mutex;

use crate::tpls::miman::type_def::Field;
handlebars_helper!(eq: |x: i32, y: i32| x == y);
handlebars_helper!(ne: |x: i32, y: i32| x != y);
handlebars_helper!(gt: |x: i32, y: i32| x > y);
handlebars_helper!(ge: |x: i32, y: i32| x >= y);
handlebars_helper!(lt: |x: i32, y: i32| x < y);
handlebars_helper!(le: |x: i32, y: i32| x <= y);
handlebars_helper!(and: |x: bool, y: bool| x && y);
handlebars_helper!(or: |x: bool, y: bool| x || y);
handlebars_helper!(not: |x: bool| x == false);
handlebars_helper!(not_empty: |x: String| !x.is_empty());
handlebars_helper!(field_access: |x: Field, y: String|  format!("{}.{}", y, x.field));

lazy_static! {
    static ref TPLS: Mutex<Handlebars<'static>> = {
        let mut reg = Handlebars::new();
        //basic
        reg.register_helper("eq", Box::new(eq));
        reg.register_helper("ne", Box::new(ne));
        reg.register_helper("gt", Box::new(gt));
        reg.register_helper("ge", Box::new(ge));
        reg.register_helper("lt", Box::new(lt));
        reg.register_helper("le", Box::new(le));
        reg.register_helper("and", Box::new(and));
        reg.register_helper("or", Box::new(or));
        reg.register_helper("not", Box::new(not));
        reg.register_helper("not_empty", Box::new(not_empty));
        //extend
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
