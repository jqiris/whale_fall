use super::http_routes::EntryGroup;
use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};
const DOCS_ITEM_TPL: &str = r#" {{name}}

> {{route_path}}

## 参数

| 字段     | 类型     | 是否必填 | 含义  |
|--------|--------|------|-----|
{{#each request}}
| {{name}} | {{type_}} | {{must}} | {{comment}} |
{{/each}}

## 响应

| 字段          | 类型     | 含义     |
|-------------|--------|--------|
{{#each response}}
| {{name}} | {{type_}} | {{comment}} |
{{/each}}

## 响应例子
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DocsItem {
    pub name: String,
    pub route_path: String,
    pub request: Vec<DocsItemField>,
    pub response: Vec<DocsItemField>,
    pub exp_json: Vec<String>,
}
impl DocsItem {
    pub fn execute(&self) -> Result<String> {
        render_template(DOCS_ITEM_TPL, self)
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DocsItemField {
    pub name: String,
    pub type_: String,
    pub must: String,
    pub comment: String,
}

const DOCS_SIDEBAR_TPL: &str = r#"
{{#each groups}}
* {{group_name}}
{{#each fun_list}}
* * [{{fun_mark}}]({uri2}})
{{/each}}
{{/each}}
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DocsSidebar {
    pub entry: String,
    pub groups: Vec<EntryGroup>,
}

impl DocsSidebar {
    pub fn execute(&self) -> Result<String> {
        render_template(DOCS_SIDEBAR_TPL, self)
    }
}
