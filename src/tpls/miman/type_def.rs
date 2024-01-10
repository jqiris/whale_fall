use anyhow::{anyhow, Result};
use handlebars::{handlebars_helper, Handlebars};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
const ENTITY_TYPE_DEF: &str = r#"
func (e *{{entity_name}}) String() string {
	data, _ := json.Marshal(e)
	return string(data)
}
func (e *{{entity_name}}) EntityName() string {
	return "{{entity_name}}"
}
func (e *{{entity_name}}) ToTagMap(tagName string) map[string]any {
	out := make(map[string]any)
	v := reflect.ValueOf(e)
	t := v.Type()
	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		name := field.Name
		val := v.FieldByName(name)
		res := strings.Split(field.Tag.Get(tagName), ",")
		_tagName := res[0]
		if _tagName != "" {
			name = _tagName
			if strings.Contains(_tagName, ";") {
				name = strings.Split(_tagName, ";")[0]
			}
		}
		out[name] = val.Interface()
	}
	return out
}

func (e *{{entity_name}}) FromMap(input map[string]any)  {
	b,_ := tools.JSONFuzzy.Marshal(input)
	tools.JSONFuzzy.Unmarshal(b,&e)
}
type {{entity_list_name}} []*{{entity_name}}
func (list {{entity_list_name}}) String() string {
	data, _ := json.Marshal(list)
	return string(data)
}
{{#each fields}}{{#if gen_slice_func}}func (list {{../entity_list_name}}) Get{{field}}List() []{{type_}} {
	return list.Get{{type_in_name}}List(func (item *{{../entity_name}}) {{type_}} {
		return {{field_access this "item"}}
	})
}{{/if}}
{{/each}}
{{#each fields}}{{#if gen_slice_func}}func (list {{../entity_list_name}}) GetMissing{{field}}List(input []{{type_}}) []{{type_}} {
    result := make([]{{type_}}, 0)
	mapping := list.Get{{field}}Map()
    for _, item := range tool_slice.Unique{{type_in_name}}Slice(input) {
        if mapping[item] == nil {
            result = append(result, item)
        }
    }
    return result
}{{/if}}
{{/each}}
{{#each fields}}{{#if gen_slice_func}}func (list {{../entity_list_name}}) FilterBy{{field}}(needle ...{{type_}}) {{../entity_list_name}} {
	if len(needle) == 0 {
		return list
	}
    
	return list.FilterBy(func (item *{{../entity_name}}) bool {
		for _, v := range needle {
			if {{field_access this "item"}} == v {
                return true
            }
        }
		return false
    })
}{{/if}}
{{/each}}
{{#each fields}}{{#if gen_slice_func}}func (list {{../entity_list_name}}) GroupBy{{field}}() map[{{type_}}]{{../entity_list_name}} {
	return list.GroupBy{{type_in_name}}(func (item *{{../entity_name}}) {{type_}} {
		return {{field_access this "item"}}
    })
}{{/if}}
{{/each}}
{{#each fields}}{{#if gen_slice_func}}func (list {{../entity_list_name}}) Get{{field}}Map() map[{{type_}}]*{{../entity_name}} {
	result := make(map[{{type_}}]*{{../entity_name}})
	for _, item := range list {
		result[{{field_access this "item"}}] = item
	}
	return result
}{{/if}}
{{/each}}
func (list {{entity_list_name}}) GetStringList(visitor func(item *{{entity_name}}) string)[]string {
	result := make([]string, 0, len(list))
	for _, item := range list {
		result = append(result, visitor(item))
	}
	return result
}
func (list {{entity_list_name}}) GetIntList(visitor func(item *{{entity_name}}) int) []int {
	result := make([]int, 0, len(list))
	for _, item := range list {
		result = append(result, visitor(item))
	}
	return result
}
func (list {{entity_list_name}}) GetInt32List(visitor func(item *{{entity_name}}) int32) []int32 {
	result := make([]int32, 0, len(list))
	for _, item := range list {
		result = append(result, visitor(item))
	}
	return result
}
func (list {{entity_list_name}}) GetInt64List(visitor func(item *{{entity_name}}) int64) []int64 {
	result := make([]int64, 0, len(list))
	for _, item := range list {
		result = append(result, visitor(item))
	}
	return result
}
func (list {{entity_list_name}}) FilterBy(visitor func(item *{{entity_name}}) bool) {{entity_list_name}} {
	result := make({{entity_list_name}}, 0, len(list))
	for _, item := range list {
		if visitor(item) {
			result = append(result, item)
		}
	}
	return result
}
func (list {{entity_list_name}}) GroupByString(visitor func(item *{{entity_name}}) string) map[string]{{entity_list_name}} {
	result := make(map[string]{{entity_list_name}})
	for _, item := range list {
		key := visitor(item)
		result[key] = append(result[key], item)
	}
	return result
}
func (list {{entity_list_name}}) GroupByInt(visitor func(item *{{entity_name}}) int) map[int]{{entity_list_name}} {
	result := make(map[int]{{entity_list_name}})
	for _, item := range list {
		key := visitor(item)
		result[key] = append(result[key], item)
	}
	return result
}
func (list {{entity_list_name}}) GroupByInt32(visitor func(item *{{entity_name}}) int32) map[int32]{{entity_list_name}} {
	result := make(map[int32]{{entity_list_name}})
	for _, item := range list {
		key := visitor(item)
		result[key] = append(result[key], item)
	}
	return result
}
func (list {{entity_list_name}}) GroupByInt64(visitor func(item *{{entity_name}}) int64) map[int64]{{entity_list_name}} {
	result := make(map[int64]{{entity_list_name}})
	for _, item := range list {
		key := visitor(item)
		result[key] = append(result[key], item)
	}
	return result
}
"#;
#[derive(Serialize, Deserialize, Default)]
pub struct EntityTypeMap {
    pub project_name: String,
    pub entity_name: String,
    pub entity_list_name: String,
    pub fields: Vec<Field>,
    pub has_creator: bool,
    pub creator_name: String,
}

impl EntityTypeMap {
    pub fn execute(&self) -> Result<String> {
        let mut reg = Handlebars::new();
        reg.register_helper("field_access", Box::new(field_access));
        match reg.render_template(ENTITY_TYPE_DEF, self) {
            Ok(output) => Ok(output),
            Err(err) => Err(anyhow!(err)),
        }
    }
}

handlebars_helper!(field_access: |x: Field, y: String|  format!("{}.{}", y, x.field));

#[derive(Serialize, Deserialize, Default)]
pub struct Field {
    pub field: String,
    pub field_tag: String,
    pub field_escaped_tag: String,
    pub field_tag_map: HashMap<String, String>,
    pub db_tag: String,
    pub json_tag: String,
    pub type_: String,
    pub use_json: bool,
    pub named_type: String,
    pub type_in_name: String,
    pub gen_slice_func: bool,
    pub nullable: bool,
    pub comparable: bool,
}
