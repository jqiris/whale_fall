use core::fmt;

use crate::{
    common::{
        file::{path_join, path_parent, rel_path},
        go::{XField, XType, XST},
        str::{first_upper_index, is_first_uppercase, parse_field_tag_map, search_index},
    },
    core::{meta::*, traits::IGenerator},
    tpls::miman::{do_def, header, type_def},
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
                list.push(self.gen_entity(pkg, &entity)?);
                let mut do_list = self.gen_do(root, pkg, &entity)?;
                list.append(&mut do_list);
            }
        }
        //micro
        if let Some(micro) = data.maps.get("micro") {
            //entity list
            let entity_list = micro.find_list_by_name("entity");
            for entity in entity_list {
                list.push(self.gen_entity(pkg, &entity)?);
                let mut do_list = self.gen_do(root, pkg, &entity)?;
                list.append(&mut do_list);
            }
        }
        Ok(list)
    }
}

impl MimanGenerator {
    fn gen_do(&self, root: &str, pkg: &str, data: &MetaNode) -> Result<Vec<GenerateData>> {
        let header_do = header::Header {
            package: "do".to_string(),
            imports: vec!["time".to_string(), "gorm.io/gorm".to_string()],
            allow_edit: false,
        };
        let path_parent = path_parent(&data.path);
        let rel_path = rel_path(root, &path_parent);
        let pkg_path = format!("{}/{}", pkg, rel_path);
        let header_conv = header::Header {
            package: "converter".to_string(),
            imports: vec![
                format!("{}/common/tools", pkg),
                format!("{}/common/core/log", pkg),
                format!("{}/common/tools/tool_time", pkg),
                format!("{}/entity", pkg_path),
                format!("{}/repo/dbal/do", pkg_path),
            ],
            allow_edit: false,
        };
        let (mut bufd, mut bufc) = (header_do.execute()?, header_conv.execute()?);
        let xst_list = data.go_struct_list();
        for xst in xst_list {
            let (_bd, _bc) = self.do_def(xst)?;
            bufd += &_bd;
            bufc += &_bc;
        }
        let list = vec![
            GenerateData {
                path: path_join(&[&path_parent, "repo", "dbal", "do", "do_gen.go"]),
                gen_type: self.generate_type(),
                out_type: OutputType::OutputTypeGo,
                content: bufd,
            },
            GenerateData {
                path: path_join(&[
                    &path_parent,
                    "repo",
                    "dbal",
                    "converter",
                    "converter_gen.go",
                ]),
                gen_type: self.generate_type(),
                out_type: OutputType::OutputTypeGo,
                content: bufc,
            },
        ];
        Ok(list)
    }
    fn gen_entity(&self, pkg: &str, data: &MetaNode) -> Result<GenerateData> {
        let mut gen_data = GenerateData {
            path: path_join(&[&data.path, "type_def_code_gen.go"]),
            gen_type: self.generate_type(),
            out_type: OutputType::OutputTypeGo,
            content: "".to_string(),
        };
        let xst_list = data.go_struct_list();
        let import = type_def::EntityTypeImport {
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
    fn do_def(&self, xst: XST) -> Result<(String, String)> {
        let mut gdo = do_def::Do {
            name: xst.name.clone(),
            fields: Vec::new(),
            delete_at: false,
        };
        let mut field_list: Vec<XField> = xst.fields.values().cloned().collect();
        field_list.sort_by(|a, b| a.idx.cmp(&b.idx));
        for field in field_list {
            let tag_desc = field.get_tag("json");
            if let Some(desc) = tag_desc {
                if desc.txt == "-" {
                    continue;
                }
                let (mut tag, mut conv_slice, mut is_point, mut ftype, mut stype, mut type2) = (
                    desc.txt,
                    false,
                    false,
                    field.xtype.clone(),
                    field.stype.clone(),
                    "".to_string(),
                );
                if desc.opts.len() > 0 {
                    if let Some(v) = desc.opts.get("conv") {
                        conv_slice = true;
                        let tag_conv = format!("conv:{}", v);
                        tag = tag.replace(format!("{};", tag_conv).as_str(), "");
                        tag = tag.replace(&tag_conv, "");
                    }
                }
                let tags = format!("`db:\"{}\" gorm:\"column:{}\"`", desc.name, tag);
                match field.stype {
                    XType::XTypeStruct => {
                        type2 = field.xtype.clone();
                        type2 = type2.replace("*", "");
                        if field.xtype.contains("time.Time") {
                            stype = XType::XTypeTime;
                        } else if field.xtype.starts_with("*") {
                            is_point = true;
                        }
                    }
                    XType::XTypeSlice => {
                        type2 = field.xtype.clone();
                        type2 = type2.replace("[]", "");
                        if type2.contains("[]") || search_index(&type2, ".") > 0 {
                            conv_slice = false
                        }
                        if is_first_uppercase(&type2) || type2.contains("map") {
                            conv_slice = false;
                        }
                        let first_upper_index = first_upper_index(&ftype);
                        if first_upper_index > -1 {
                            let index = first_upper_index as usize;
                            ftype = format!("{}entity.{}", &ftype[..index], &ftype[index..]);
                        }
                    }
                    XType::XTypeMap => {
                        let first_upper_index = first_upper_index(&ftype);
                        if first_upper_index > -1 {
                            let index = first_upper_index as usize;
                            ftype = format!("{}entity.{}", &ftype[..index], &ftype[index..]);
                        }
                    }
                    _ => {}
                }
                let do_field = do_def::DoField {
                    name: field.name,
                    type_: ftype,
                    type2,
                    stype: stype as i32,
                    tag: tags,
                    conv_slice,
                    is_point,
                    comment: field.comment,
                };
                gdo.fields.push(do_field);
            }
        }
        if gdo.fields.len() == 0 {
            return Ok(("".to_string(), "".to_string()));
        }
        let buf = gdo.execute()?;
        let do_conv = do_def::DoConv {
            name: gdo.name,
            fields: gdo.fields,
        };
        let buf2 = do_conv.execute()?;
        Ok((buf, buf2))
    }
    fn type_def(&self, pkg: &str, xst: XST) -> Result<String> {
        let mut tpl = type_def::EntityTypeMap {
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
            let mut fe = type_def::Field {
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
