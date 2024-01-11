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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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

const CONV_DO_TPL: &str = r#"

func From{{name}}Entity(input *entity.{{name}}) *do.{{name}}Do{
	if input == nil {
		return nil
	}
	output := &do.{{name}}Do{}
{{#each fields }}
	{{#if (eq stype 1)}} 
		{{#if is_point}} 
	if input.{{name}} != nil {
		b, _ := tools.JSON.Marshal(input.{{name}})
		output.{{name}} = string(b)
	}
		{{else}}
	b, _ := tools.JSON.Marshal(input.{{name}})
	output.{{name}} = string(b)
		{{/if}}
	{{/if}}
	{{#if (eq stype 2)}}
	if input.{{name}} != nil {
		{{#if conv_slice}}
			output.{{name}} = slice_utils.Implode(input.{{name}},",")
		{{else}}
		b, _ := tools.JSON.Marshal(input.{{name}})
		output.{{name}} = string(b)
		{{/if}}
	}
    {{/if}}
	{{#if (eq stype 3)}}
	if input.{{name}} != nil {
		b, _ := tools.JSON.Marshal(input.{{name}})
		output.{{name}} = string(b)
	}
    {{/if}}
	{{#if (eq stype 4)}}
		if !input.{{name}}.IsZero() {
			output.{{name}} = &input.{{name}}
		}
    {{/if}}
	{{#if (eq stype 0)}}
	output.{{name}} = input.{{name}}
	{{/if}}
{{/each}}
	return output
}

func To{{name}}Entity(input *do.{{name}}Do) *entity.{{name}}{
	if input == nil {
		return nil
	}
	output := &entity.{{name}}{}
{{#each fields }}
	{{#if (eq stype 1)}} 
	if input.{{name}} != ""  {
		{{#if is_point}} 
		t := &entity.{{ type2}}{}
		{{else}}
		t := entity.{{ type2}}{}
		{{/if}}
		err := tools.JSON.Unmarshal([]byte(input.{{name}}), &t)
		if err != nil {
			log.Errorf("converter To{{$name}}Entity[{{name}}] err %v", err)
		} else {
			output.{{name}} = t
		}
	}
    {{/if}}
	{{#if (eq stype 2)}}
		if input.{{name}} != "" {
			{{#if conv_slice}}
				{{#if (eq type2 "int64") }}
					output.{{name}} = slice_utils.ExplodeInt64(input.{{name}},",")
                {{/if}}
				{{#if (eq type2 "int") }}
					output.{{name}} = slice_utils.ExplodeInt(input.{{name}},",")
                {{/if}}
				{{#if (and (ne type2 "int64") (ne type2 "int")) }}
					output.{{name}} = slice_utils.ExplodeStr(input.{{name}},",")
				{{/if}}
			{{else}}
				t := {{type_}}{}
				err := tools.JSON.Unmarshal([]byte(input.{{name}}), &t)
				if err != nil {
					log.Errorf("converter To{{$name}}Entity[{{name}}] err %v", err)
				} else {
					output.{{name}} = t
				}
			{{/if}}
		}
    {{/if}}
	{{#if (eq stype 3)}}
		if input.{{name}} != "" {
			t := {{type_}}{}
			err := tools.JSON.Unmarshal([]byte(input.{{name}}), &t)
			if err != nil {
				log.Errorf("converter To{{$name}}Entity[{{name}}] err %v", err)
			} else {
				output.{{name}} = t
			}
		}
    {{/if}}
	{{#if (eq stype 4)}}
		if input.{{name}} != nil {
			output.{{name}} = *input.{{name}}
		}
    {{/if}}
	{{#if (eq stype 0)}}
	output.{{name}} = input.{{name}}
	{{/if}}
{{/each}}
	return output
}

func From{{name}}List(input entity.{{name}}List) do.{{name}}DoList {
	if input == nil {
		return nil
	}
	output := make([]*do.{{name}}Do, 0, len(input))
	for _, item := range input {
		resultItem := From{{name}}Entity(item)
		output = append(output, resultItem)
	}
	return output
}

func To{{name}}List(input do.{{name}}DoList) entity.{{name}}List {
	if input == nil || len(input) == 0 {
		return nil
	}
	output := make(entity.{{name}}List, 0, len(input))
	for _, item := range input {
		resultItem := To{{name}}Entity(item)
		output = append(output, resultItem)
	}
	return output
}
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DoConv {
    pub name: String,
    pub fields: Vec<DoField>,
}

impl DoConv {
    pub fn execute(&self) -> Result<String> {
        render_template(CONV_DO_TPL, self)
    }
}
