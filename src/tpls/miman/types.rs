use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

const IO_TPL: &str = r#"
type {{name}} struct{
{{#each fields}}
    {{name}} {{type_}} {{{tag}}} // {{comment}}
{{/each}}
}
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IO {
    pub name: String,
    pub fields: Vec<IoField>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct IoField {
    pub name: String,
    pub type_: String,
    pub type2: String,
    pub type2_entity: bool,
    pub stype: i32,
    pub tag: String,
    pub hidden: bool,
    pub comment: String,
}

impl IO {
    pub fn execute(&self) -> Result<String> {
        render_template(IO_TPL, self)
    }
}

const CONV_IO_TPL: &str = r#"
func From{{name_mark}}{{name}}Entity(input *entity.{{name}}) *types.{{name}}{
	if input == nil {
		return nil
	}
	output := &types.{{name}}{}
{{#each fields }}
	{{#if (eq stype 1)}}
	output.{{name}} = From{{../name_mark}}{{type2}}Entity(input.{{name}})
    {{/if}}
	{{#if (eq stype 2)}}
	if input.{{name}} != nil {
		{{#if type2_entity}}
		output.{{name}} = From{{../name_mark}}{{type2}}List(input.{{name}})
		{{else}}
		output.{{name}} = input.{{name}}
		{{/if}}
	}
    {{/if}}
	{{#if (eq stype 4)}}
		if !input.{{name}}.IsZero() {
			output.{{name}} = tool_time.TimeToDateTimeString(input.{{name}})
		}
    {{/if}}
	{{#if (and (ne stype 1) (ne stype 2) (ne stype 4))}}
	output.{{name}} = input.{{name}}
	{{/if}}
{{/each}}
	return output
}

func To{{name_mark}}{{name}}Entity(input *types.{{name}}) *entity.{{name}}{
	if input == nil {
		return nil
	}
	output := &entity.{{name}}{}
{{#each fields }}
	{{#if (eq stype 1)}} 
	output.{{name}} = To{{../name_mark}}{{type2}}Entity(input.{{name}})
    {{/if}}
	{{#if (eq stype 2)}}
		{{#if type2_entity}}
		output.{{name}} = To{{../name_mark}}{{type2}}List(input.{{name}})
		{{else}}
		output.{{name}} = input.{{name}}
		{{/if}}
    {{/if}}
	{{#if (eq stype 4)}}
		if ts := tool_time.ParseDateTime(input.{{name}}); !ts.IsZero() {
			output.{{name}} = ts
		}
    {{/if}}
	{{#if (and (ne stype 1) (ne stype 2) (ne stype 4))}}
	output.{{name}} = input.{{name}}
	{{/if}}
{{/each}}
	return output
}

func From{{name_mark}}{{name}}List(input entity.{{name}}List) []*types.{{name}} {
	if input == nil {
		return nil
	}
	output := make([]*types.{{name}}, 0, len(input))
	for _, item := range input {
		resultItem := From{{name_mark}}{{name}}Entity(item)
		output = append(output, resultItem)
	}
	return output
}

func To{{name_mark}}{{name}}List(input []*types.{{name}}) entity.{{name}}List {
	if input == nil || len(input) == 0 {
		return nil
	}
	output := make(entity.{{name}}List, 0, len(input))
	for _, item := range input {
		resultItem := To{{name_mark}}{{name}}Entity(item)
		output = append(output, resultItem)
	}
	return output
}

"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IoConv {
    pub src_path: String,
    pub name: String,
    pub name_mark: String,
    pub package: String,
    pub imports: Vec<String>,
    pub fields: Vec<IoField>,
}

impl IoConv {
    pub fn execute(&self) -> Result<String> {
        render_template(CONV_IO_TPL, self)
    }
}
