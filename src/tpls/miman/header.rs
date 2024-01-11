use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

const HEADER_TPL: &str = r#"{{#if (not allow_edit)}}// Code generated by whale_fall. DO NOT EDIT.{{/if}}
package {{package}}

import (
{{#each imports }}
"{{{this}}}"
{{/each}}
)
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Header {
    pub package: String,
    pub imports: Vec<String>,
    pub allow_edit: bool,
}

impl Header {
    pub fn execute(&self) -> Result<String> {
        render_template(HEADER_TPL, self)
    }
}
