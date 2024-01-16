use crate::common::go::{XField, XType, XST};
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn gen_json_exp(fields: &HashMap<String, XField>, struct_list: &HashMap<String, XST>) -> Value {
    let mut maps = HashMap::new();
    for (_, field) in fields {
        if let Some(j) = field.get_tag("json") {
            let (k, v) = (j.name, gen_json_val(field, struct_list));
            maps.insert(k, v);
        }
    }
    json!(maps)
}
pub fn gen_json_val(field: &XField, struct_list: &HashMap<String, XST>) -> Value {
    match field.stype {
        XType::XTypeStruct => {
            let sk = field.xtype.trim_start_matches('*').to_string();
            if let Some(v) = struct_list.get(&sk) {
                return gen_json_exp(&v.fields, struct_list);
            }
        }
        XType::XTypeSlice => {
            let sk = field.xtype.replace("*", "").replace("[]", "");
            let mut list = Vec::new();
            if let Some(v) = struct_list.get(&sk) {
                list.push(gen_json_exp(&v.fields, struct_list));
            } else {
                list.push(gen_json_zero_val(&field.xtype));
            }
            return json!(list);
        }
        _ => {
            return gen_json_zero_val(&field.xtype);
        }
    }
    json!("")
}

pub fn gen_json_zero_val(x_type: &str) -> Value {
    match x_type {
        "bool" => json!(true),
        x if x.contains("int") => json!(0),
        x if x.contains("float") => json!(0.1),
        _ => json!(""),
    }
}

pub fn sort_fields(fields: &HashMap<String, XField>) -> Vec<XField> {
    let mut r = Vec::new();
    for field in fields.values() {
        r.push(field.clone());
    }
    r.sort_by(|a, b| a.idx.cmp(&b.idx));
    r
}
