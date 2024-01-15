use super::http_routes::EntryFunItem;
use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

const HANDLER_FUNC_INIT_TPL: &str = r#"package handler

import (
	"context"

	"{{entry_path}}/types"
)
{{#each fun_list}}
func {{fun_name}}(ctx context.Context, req *types.{{req_name}}) (*types.{{resp_name}}, error) {
	var (
		resp = &types.{{resp_name}}{}
	)
	// todo ...
	return resp, nil
}
{{/each}}
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HandlerFunc {
    pub entry_path: String,
    pub entry: String,
    pub group: String,
    pub fun_list: Vec<EntryFunItem>,
}

impl HandlerFunc {
    pub fn execute(&self) -> Result<String> {
        render_template(HANDLER_FUNC_INIT_TPL, self)
    }
}

const HANDLER_FUNC_APPEND_TPL: &str = r#"
{{{body}}}
{{#each fun_list}}
func {{fun_name}}(ctx context.Context, req *types.{{req_name}}) (*types.{{resp_name}}, error) {
	var (
		resp = &types.{{resp_name}}{}
	)
	// todo ...
	return resp, nil
}
{{/each}}
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HandlerFuncAppend {
    pub body: String,
    pub fun_list: Vec<EntryFunItem>,
}

impl HandlerFuncAppend {
    pub fn execute(&self) -> Result<String> {
        render_template(HANDLER_FUNC_APPEND_TPL, self)
    }
}
