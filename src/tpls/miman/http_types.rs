use super::http_routes::EntryFunItem;
use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

const HTTP_TYPES_TPL: &str = r#"
package types
{{#each fun_list}}
// {{req_name}} {{fun_mark}}参数 
type {{req_name}} struct {}
// {{resp_name}} {{fun_mark}}响应
type {{resp_name}} struct {}
{{/each}}
"#;

const HTTP_TYPES_APPEND_TPL: &str = r#"
{{{body}}}
{{#each fun_list}}
// {{req_name}} {{fun_mark}}参数 
type {{req_name}} struct {}
// {{resp_name}} {{fun_mark}}响应
type {{resp_name}} struct {}
{{/each}}
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HandlerTypes {
    pub entry_path: String,
    pub entry: String,
    pub group: String,
    pub fun_list: Vec<EntryFunItem>,
}

impl HandlerTypes {
    pub fn execute(&self) -> Result<String> {
        render_template(HTTP_TYPES_TPL, self)
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HandlerTypesAppend {
    pub body: String,
    pub fun_list: Vec<EntryFunItem>,
}

impl HandlerTypesAppend {
    pub fn execute(&self) -> Result<String> {
        render_template(HTTP_TYPES_APPEND_TPL, self)
    }
}
