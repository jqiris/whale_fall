use super::micro_entry::MicroFunItem;
use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const MICRO_SERVICE_FUNC_INIT_TPL: &str = r#"package service

import (
	"context"

	"{{app_pkg_name}}/types_{{app_name}}"
)

// {{service}} @GI
type {{service}} struct {
}

func New{{service}}() *{{service}} {
	return &{{service}}{}
}

{{#each fun_list}}
// {{method}} {{fun_mark}} 
func (s *{{service}}){{method}}(ctx context.Context, input *types_{{../app_name}}.{{req_name}}) (*types_{{../app_name}}.{{resp_name}}, error) {
	var (
		output = &types_{{../app_name}}.{{resp_name}}{}
	)

	// todo ...
	return output, nil
}
{{/each}}

"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MicroServiceFunc {
    pub app_name: String,
    pub app_pkg_name: String,
    pub service: String,
    pub fun_list: Vec<MicroFunItem>,
}

impl MicroServiceFunc {
    pub fn execute(&self) -> Result<String> {
        render_template(MICRO_SERVICE_FUNC_INIT_TPL, self)
    }
}

pub const MICRO_SERVICE_FUNC_APPEND_TPL: &str = r#"
{{{body}}}
{{#each fun_list}}
func (s *{{service}}){{method}}(ctx context.Context, input *types_{{../app_name}}.{{req_name}}) (*types_{{../app_name}}.{{resp_name}}, error) {
	var (
		output = &types_{{../app_name}}.{{resp_name}}{}
	)

	// todo ...
	return output, nil
}
{{/each}}
"#;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MicroServiceAppend {
    pub body: String,
    pub app_name: String,
    pub fun_list: Vec<MicroFunItem>,
}

impl MicroServiceAppend {
    pub fn execute(&self) -> Result<String> {
        render_template(MICRO_SERVICE_FUNC_APPEND_TPL, self)
    }
}
