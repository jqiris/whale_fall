use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::tpls::engine::render_template;

const DO_TPL: &str = r#"
type {{name}}Do struct {
    {{#each fields}}
        {{#if (eq stype 4)}}
        {{name}} *{{type_}} {{tag}} // {{comment}}
        {{/if}}
        {{#if (and (gt stype 0) (lt stype 4))}}
        {{name}} string {{tag}} // {{comment}}
        {{/if}}
        {{#if (or (le stype 0) (gt stype 4))}}
        {{name}} {{type_}} {{tag}} // {{comment}}
        {{/if}}
    {{/each}}
        {{#if delete_at}}
        DeletedAt gorm.DeletedAt ` + "`" + `db:"deleted_at" gorm:"column:deleted_at"` + "`" + ` // 软删除标识
        {{/if}}
    }
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Do {
    pub name: String,
    pub fields: Vec<DoField>,
    pub delete_at: bool,
}

impl Do {
    pub fn execute(&self) -> Result<String> {
        render_template(DO_TPL, self)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DoField {
    pub name: String,
    pub type_: String,
    pub type2: String,
    pub stype: i32,
    pub tag: String,
    pub conv_slice: bool,
    pub is_point: bool,
    pub comment: String,
}
