use super::micro_entry::MicroFunItem;
use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

const MICRO_TYPES_TPL: &str = r#"
package types_{{app_name}}

import (
	"time"
)

{{#each fun_list}}

// {{req_name}} .
type {{req_name}} struct {}

// {{resp_name}} .
type {{resp_name}} struct {}

{{/each}}
"#;

const MICRO_TYPES_APPEND_TPL: &str = r#"
{{body}}
{{#each fun_list}}

// {{req_name}} .
type {{req_name}} struct {}

// {{resp_name}} .
type {{resp_name}} struct {}

{{/each}}
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MicroTypesAppend {
    pub body: String,
    pub fun_list: Vec<MicroFunItem>,
}

impl MicroTypesAppend {
    pub fn execute(&self) -> Result<String> {
        render_template(MICRO_TYPES_APPEND_TPL, self)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MicroTypes {
    pub app_name: String,
    pub app_pkg_path: String,
    pub fun_list: Vec<MicroFunItem>,
}

impl MicroTypes {
    pub fn execute(&self) -> Result<String> {
        render_template(MICRO_TYPES_TPL, self)
    }
}
