use crate::{
    common::file::path_join,
    core::{meta::*, traits::IGenerator},
    tpls::sgz::{dao_def::Dao, do_def::Do, logic_def::Logic, svc_def::Svc, EntityField},
    Basic,
};
use anyhow::Result;
use core::fmt;
use regex::Regex;
use std::{collections::HashMap, fs};
pub struct SgzGenerator {}

impl fmt::Display for SgzGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sgz")
    }
}

impl IGenerator for SgzGenerator {
    fn generate_type(&self) -> GenerateType {
        GenerateType::GenerateTypeSgz
    }

    fn generate(&self, basic: &Basic, meta: &MetaNode) -> Result<Vec<GenerateData>> {
        let file = &basic.file;
        let mut list = Vec::new();
        if let Some(entity) = meta.find_by_name("entity") {
            if let Some(data) = entity.find_by_name(file) {
                list.extend(self.gen_dbs(basic, meta, &data)?);
            }
        }
        Ok(list)
    }
}

impl SgzGenerator {
    pub fn gen_dbs(
        &self,
        basic: &Basic,
        meta: &MetaNode,
        data: &MetaNode,
    ) -> Result<Vec<GenerateData>> {
        let services = meta.find_by_name("service");
        let mut list = Vec::new();
        let st_list = data.go_struct_list();
        for st in st_list.iter() {
            let name = st.name.clone();
            let mut sname = "".to_string();
            let mut table = "".to_string();
            let mut group = "".to_string();
            let mut prikey = "".to_string();
            let mut priname = "".to_string();
            let mut logic_dir = "".to_string();
            let mut logic_package = "".to_string();
            let mut service_file = "".to_string();
            for comment in &st.docs {
                let data = self.parse_comment(&comment.text);
                if let Some(v) = data.get("sname") {
                    sname = v.to_owned();
                }
                if let Some(v) = data.get("table") {
                    table = v.to_owned();
                }
                if let Some(v) = data.get("group") {
                    group = v.to_owned();
                }
                if let Some(v) = data.get("primary_key") {
                    prikey = v.to_owned();
                }
                if let Some(v) = data.get("primary_name") {
                    priname = v.to_owned();
                }
                if let Some(v) = data.get("logic_dir") {
                    logic_dir = v.to_owned();
                }
                if let Some(v) = data.get("logic_package") {
                    logic_package = v.to_owned();
                }
                if let Some(v) = data.get("service_file") {
                    service_file = v.to_owned();
                }
            }
            let mut fields: Vec<EntityField> = Vec::new();

            for (_, field) in &st.fields {
                let mut sname = "".to_string();
                if let Some(tag) = field.get_tag("json") {
                    sname = tag.name.to_owned();
                }
                let f = EntityField {
                    name: field.name.clone(),
                    sname,
                    comment: field.comment.clone(),
                    type_name: field.xtype.clone(),
                };
                fields.push(f);
            }
            fields.sort_by(|a, b| a.name.cmp(&b.name));
            let dod = Do {
                package: basic.package.clone(),
                entity_name: name.clone(),
                table: table.clone(),
                fields: fields.clone(),
            };
            let buf = dod.execute()?;
            list.push(GenerateData {
                path: path_join(&[&basic.root, "internal", "model", "do", &data.name]),
                gen_type: GenerateType::GenerateTypeSgz,
                out_type: OutputType::OutputTypeGo,
                content: buf,
                ..Default::default()
            });
            let dao = Dao {
                package: basic.package.clone(),
                entity_name: name.clone(),
                entity_sname: sname,
                group,
                table,
                prikey,
                fields,
            };
            let buf = dao.execute()?;
            list.push(GenerateData {
                path: path_join(&[&basic.root, "internal", "dao", "internal", &data.name]),
                gen_type: GenerateType::GenerateTypeSgz,
                out_type: OutputType::OutputTypeGo,
                content: buf,
                ..Default::default()
            });
            let buf = dao.execute_single()?;
            list.push(GenerateData {
                path: path_join(&[&basic.root, "internal", "dao", &data.name]),
                gen_type: GenerateType::GenerateTypeSgz,
                out_type: OutputType::OutputTypeGo,
                content: buf,
                ..Default::default()
            });

            let logic = Logic {
                package: basic.package.clone(),
                entity_name: name.clone(),
                pkg_name: logic_package,
                pri_name: priname,
            };
            let buf = logic.execute()?;
            list.push(GenerateData {
                path: path_join(&[&basic.root, "internal", "logic", &logic_dir, &data.name]),
                gen_type: GenerateType::GenerateTypeSgz,
                out_type: OutputType::OutputTypeGo,
                content: buf,
                ..Default::default()
            });
            //service
            if let Some(service) = &services {
                let svc_name = format!("I{}", st.name);
                if service.find_go_inf(&svc_name).is_none() {
                    let file_path = path_join(&[&basic.root, "internal", "service", &service_file]);
                    let mut content = "".to_string();
                    if let Ok(file_data) = fs::read_to_string(&file_path) {
                        content = format!("{}\n", file_data);
                    }
                    let svc = Svc {
                        package: basic.package.clone(),
                        entity_name: name,
                    };
                    let buf = svc.execute()?;
                    let content = format!("{}{}", content, buf);
                    list.push(GenerateData {
                        path: file_path,
                        gen_type: GenerateType::GenerateTypeSgz,
                        out_type: OutputType::OutputTypeGo,
                        content,
                        ..Default::default()
                    });
                }
            }
        }
        Ok(list)
    }
    fn parse_comment(&self, comment: &str) -> HashMap<String, String> {
        let re = Regex::new(r"@(\w+)\[(.*?)\]").unwrap();
        let mut data = HashMap::new();

        for cap in re.captures_iter(comment) {
            let key = cap.get(1).unwrap().as_str().to_string();
            let value = cap.get(2).unwrap().as_str().to_string();
            data.insert(key, value);
        }

        data
    }
}
