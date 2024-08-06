use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::EntityField;
const DO_TPL: &str = r#"
package do

import (
	"github.com/gogf/gf/v2/frame/g"
	"{{package}}/internal/ml"
	"{{package}}/internal/model/entity"
)

// {{entity_name}}
type {{entity_name}} struct {
	g.Meta     `orm:"table:{{table}}, do:true"`
{{#each fields}}
    {{name}}   interface{} // {{comment}}
{{/each}}
}

type {{entity_name}}ListInput struct {
	ml.BaseList
	Where {{entity_name}} // 查询条件
}

type {{entity_name}}ListOutput struct {
	Items   []*entity.{{entity_name}} // 列表
	Page    int                  // 分页号码
	Total   int                  // 总页数
	Records int                  // 数据总数
	Size    int                  // 单页数量
}

type {{entity_name}}ListKeyOutput struct {
	Items   []interface{} // 列表
	Page    int           // 分页号码
	Total   int           // 总页数
	Records int           // 数据总数
	Size    int           // 单页数量
}
"#;

#[derive(Serialize, Deserialize, Default)]
pub struct Do {
    pub package: String,
    pub entity_name: String,
    pub table: String,
    pub fields: Vec<EntityField>,
}

impl Do {
    pub fn execute(&self) -> Result<String> {
        render_template(DO_TPL, self)
    }
}
