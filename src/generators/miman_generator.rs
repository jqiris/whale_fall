use core::fmt;

use crate::{
    common::{
        file::path_join,
        go::{XType, XST},
        str::parse_field_tag_map,
    },
    core::{meta::*, traits::IGenerator},
    tpls::miman::type_def::*,
};
use anyhow::Result;
pub struct MimanGenerator {}

impl fmt::Display for MimanGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "miman")
    }
}

impl IGenerator for MimanGenerator {
    fn generate_type(&self) -> GenerateType {
        GenerateType::GenerateTypeMiman
    }

    fn generate(&self, root: &str, pkg: &str, data: ProcessData) -> Result<Vec<GenerateData>> {
        let mut list = Vec::new();
        //business
        if let Some(buiness) = data.maps.get("business") {
            //entity list
            let entity_list = buiness.find_list_by_name("entity");
            for entity in entity_list {
                list.push(self.gen_entity(pkg, entity)?);
            }
        }
        //micro
        if let Some(micro) = data.maps.get("micro") {
            //entity list
            let entity_list = micro.find_list_by_name("entity");
            for entity in entity_list {
                list.push(self.gen_entity(pkg, entity)?);
            }
        }
        Ok(list)
    }
}

impl MimanGenerator {
    fn gen_entity(&self, pkg: &str, data: MetaNode) -> Result<GenerateData> {
        let mut gen_data = GenerateData {
            path: path_join(&[&data.path, "type_def_code_gen.go"]),
            gen_type: self.generate_type(),
            out_type: OutputType::OutputTypeGo,
            content: "".to_string(),
        };
        let xst_list = data.go_struct_list();
        let import = EntityTypeImport {
            project_name: pkg.to_string(),
            package_name: "entity".to_string(),
        };
        let mut buf = import.execute()?;
        for xst in xst_list {
            buf += &self.type_def(pkg, xst)?;
        }
        gen_data.content = buf;
        Ok(gen_data)
    }
    fn type_def(&self, pkg: &str, xst: XST) -> Result<String> {
        let mut tpl = EntityTypeMap {
            project_name: pkg.to_string(),
            entity_name: xst.name.clone(),
            entity_list_name: format!("{}List", xst.name),
            fields: Vec::new(),
            has_creator: false,
            creator_name: "".to_string(),
        };
        let mut field_list = Vec::new();
        for (_, field) in xst.fields {
            field_list.push(field);
        }
        field_list.sort_by(|a, b| a.idx.cmp(&b.idx));

        let mut fe_list = Vec::new();
        for field in field_list {
            let type_ = field.xtype.clone();
            let tags = field.tag.trim_matches('`');
            let tags_map = parse_field_tag_map(tags);
            let json_tag = tags_map.get("json").unwrap_or(&"".to_string()).clone();
            let mut db_tag = tags_map.get("db").unwrap_or(&"".to_string()).clone();
            if db_tag.len() > 0 && db_tag.contains(";") {
                db_tag = db_tag.split(";").collect::<Vec<&str>>()[0].to_string();
            }
            if db_tag == "create_time"
                || db_tag == "update_time"
                || db_tag == "id"
                || db_tag == "deleted_at"
            {
                db_tag = "".to_string();
            }
            let mut fe = Field {
                field: field.name.clone(),
                field_tag: tags.to_string(),
                field_escaped_tag: format!("{:?}", tags),
                field_tag_map: tags_map,
                db_tag,
                json_tag,
                type_: type_.clone(),
                use_json: false,
                named_type: "".to_string(),
                type_in_name: "".to_string(),
                gen_slice_func: true,
                nullable: false,
                comparable: false,
            };
            if field.stype != XType::XTypeBasic && field.stype != XType::XTypeTime {
                fe.use_json = true;
            }
            if !type_.contains("*") || field.stype >= XType::XTypeSlice || type_ == "interface{}" {
                fe.nullable = true;
            } else {
                fe.comparable = true;
            }
            match type_.as_str() {
                "int" => {
                    fe.type_in_name = "Int".to_string();
                }
                "int32" => {
                    fe.type_in_name = "Int32".to_string();
                }
                "int64" => {
                    fe.type_in_name = "Int64".to_string();
                }
                "string" => {
                    fe.type_in_name = "String".to_string();
                }
                _ => {
                    fe.gen_slice_func = false;
                }
            };
            fe_list.push(fe);
        }
        tpl.fields = fe_list;
        tpl.execute()
    }
}
