use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::Path,
    vec,
};

use crate::{
    common::{
        file::{path_join, path_name, path_parent, rel_path},
        go::{XField, XType, XST},
        str::*,
    },
    core::{meta::*, traits::IGenerator},
    tpls::miman::{
        dao_def, do_def, docs, gi_def, handler, header, http_routes, http_types, micro_entry,
        micro_provider, micro_service, micro_types, repo_def, type_def, types,
    },
};
use anyhow::{Ok, Result};
use regex::Regex;
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
        let mut micro_apps = Vec::new();
        //micro
        if let Some(micro) = data.maps.get("micro") {
            //entity list
            let entity_list = micro.find_list_by_name("entity");
            for entity in entity_list {
                list.push(self.gen_entity(pkg, &entity)?);
                let mut do_list = self.gen_do(root, pkg, &entity)?;
                list.append(&mut do_list);
                let mut repo_list = self.gen_repos(root, pkg, &entity)?;
                list.append(&mut repo_list);
            }
            //do list
            let do_list = micro.find_list_by_name("do");
            for ido in do_list {
                let mut do_next_list = self.gen_do_next(root, pkg, &ido)?;
                list.append(&mut do_next_list);
            }
            let mut app_provider = Vec::new();
            //micro func
            micro_apps = micro.get_dir_childs();
            for micro_app in &micro_apps {
                let (need_provide, mut func_list) = self.gen_micro_func(root, pkg, &micro_app)?;
                list.append(&mut func_list);
                if need_provide {
                    app_provider.push(to_upper_first(&micro_app.name));
                }
            }
            //micro provider
            if app_provider.len() > 0 {
                let tpl = micro_provider::MicroProvider {
                    micro_list: app_provider,
                };
                list.push(GenerateData {
                    path: path_join(&[root, "provider", "provider_gen.go"]),
                    gen_type: self.generate_type(),
                    out_type: OutputType::OutputTypeGo,
                    content: tpl.execute()?,
                });
            }
        }
        //business
        if let Some(buiness) = data.maps.get("business") {
            //entity list
            let entity_list = buiness.find_list_by_name("entity");
            for entity in entity_list {
                list.push(self.gen_entity(pkg, &entity)?);
                let mut do_list = self.gen_do(root, pkg, &entity)?;
                list.append(&mut do_list);
                let mut repo_list = self.gen_repos(root, pkg, &entity)?;
                list.append(&mut repo_list);
            }
            //do list
            let do_list = buiness.find_list_by_name("do");
            for ido in do_list {
                let mut do_next_list = self.gen_do_next(root, pkg, &ido)?;
                list.append(&mut do_next_list);
            }
            //app list
            let apps = buiness.get_dir_childs();
            for app in apps {
                if let Some(cmd) = app.find_by_name("cmd") {
                    let modules = cmd.get_dir_childs();
                    for module in modules {
                        //app types
                        let mut app_types =
                            self.gen_app_types(root, pkg, &app, &module, &micro_apps)?;
                        list.append(&mut app_types);
                        //app handlers
                        let mut app_handlers = self.gen_app_handlers(root, pkg, &app, &module)?;
                        list.append(&mut app_handlers);
                    }
                }
            }
        }
        //gi
        if let Some(gi_list) = data.lists.get("gi") {
            for gi in gi_list {
                list.push(self.gen_gi(&gi)?);
            }
        }
        Ok(list)
    }
}

impl MimanGenerator {
    fn gen_app_handlers(
        &self,
        root: &str,
        pkg: &str,
        app: &MetaNode,
        module: &MetaNode,
    ) -> Result<Vec<GenerateData>> {
        let mut list = Vec::new();
        let app_rel_path = rel_path(root, &app.path);
        let app_pkg_path = format!("{}/{}", pkg, app_rel_path);
        let app_name = app.name.clone();
        let module_rel_path = rel_path(root, &module.path);
        let module_pkg_path = format!("{}/{}", pkg, module_rel_path);
        let module_name = module.name.clone();
        if let Some(route) = module.find_by_name("route") {
            if let Some(routes) = route.find_go_func("_gen") {
                if routes.comment.len() > 0 {
                    let groups = self.biz_entry_doc_parse(&module.name, &routes.comment, false);
                    let ef = http_routes::HttpEntry {
                        project: pkg.to_string(),
                        app_name: app_name.clone(),
                        app_name_uf: to_upper_first(&app_name),
                        app_pkg_path,
                        entry_name: module_name,
                        entry_path: module.path.clone(),
                        entry_pkg_path: module_pkg_path,
                        groups,
                        ..Default::default()
                    };
                    let buf = ef.execute(http_routes::HTTP_ROUTE_TPL)?;
                    list.push(GenerateData {
                        path: path_join(&[&module.path, "route", "routes_gen.go"]),
                        gen_type: GenerateType::GenerateTypeMiman,
                        out_type: OutputType::OutputTypeGo,
                        content: buf,
                    });
                    let mut handler_list = self.gen_route_handler(module, &ef)?;
                    list.append(&mut handler_list);
                    let mut types_list = self.gen_route_types(module, &ef)?;
                    list.append(&mut types_list);
                    let mut docs_list = self.gen_route_docs(root, app, module, &ef)?;
                    list.append(&mut docs_list);
                }
            }
        }
        Ok(list)
    }

    fn gen_route_docs(
        &self,
        root: &str,
        app: &MetaNode,
        module: &MetaNode,
        ef: &http_routes::HttpEntry,
    ) -> Result<Vec<GenerateData>> {
        let mut list = Vec::new();
        let mut xst_maps = HashMap::new();
        if let Some(types) = module.find_by_name("types") {
            xst_maps = types.go_struct_maps();
            for dir in types.get_dir_childs() {
                let dir_maps = dir.go_struct_maps();
                for (s, xst) in dir_maps {
                    xst_maps.insert(format!("{}.{}", dir.name, s), xst);
                }
            }
        }
        for group in &ef.groups {
            let group_uri_name = to_snake_case(&group.group).replace("_", "-");
            let dir = path_join(&[
                root,
                "panel",
                "docs",
                &app.name,
                &module.name,
                &group_uri_name,
            ]);
            for fun in &group.fun_list {
                if xst_maps.contains_key(&fun.req_name) {
                    list.push(self.docs_item(&dir, fun, &xst_maps)?);
                }
            }
        }
        let filename = path_join(&[
            root,
            "panel",
            "docs",
            &app.name,
            &module.name,
            "_sidebar.md",
        ]);
        let _t = docs::DocsSidebar {
            entry: ef.entry_name.clone(),
            groups: ef.groups.clone(),
        };
        let buf = _t.execute()?;
        list.push(GenerateData {
            path: filename,
            gen_type: GenerateType::GenerateTypeMiman,
            out_type: OutputType::OutputTypeMd,
            content: buf,
        });
        Ok(list)
    }

    pub fn docs_item(
        &self,
        dir: &str,
        f: &http_routes::EntryFunItem,
        struct_list: &HashMap<String, XST>,
    ) -> Result<GenerateData> {
        let uris = f.uri2.split("/").collect::<Vec<&str>>();
        let filename = path_join(&[&dir, &format!("{}.md", uris.last().unwrap_or(&""))]);
        let req_xst = struct_list
            .get(&f.req_name)
            .unwrap_or_else(|| panic!("Missing request struct: {}", f.req_name));
        let resp_xst = struct_list
            .get(&f.resp_name)
            .unwrap_or_else(|| panic!("Missing response struct: {}", f.resp_name));
        let request = self.to_docs_item_fields(&req_xst.fields, struct_list, String::new());
        let response = self.to_docs_item_fields(&resp_xst.fields, struct_list, String::new());
        let mut _t = docs::DocsItem {
            name: f.fun_mark.clone(),
            route_path: f.uri.clone(),
            request,
            response,
            exp_json: Vec::new(),
        };
        let body = self.get_json(&resp_xst.fields, struct_list);
        _t.exp_json.push("```\n".to_string());
        _t.exp_json.push(body);
        _t.exp_json.push("\n```".to_string());
        let buf = _t.execute()?;
        Ok(GenerateData {
            path: filename,
            gen_type: GenerateType::GenerateTypeMiman,
            out_type: OutputType::OutputTypeMd,
            content: buf,
        })
    }
    fn get_json(
        &self,
        fields: &HashMap<String, XField>,
        struct_list: &HashMap<String, XST>,
    ) -> String {
        let mut list = Vec::new();
        for (_, field) in fields {
            if let Some(j) = field.get_tag("json") {
                let line = format!("\"{}\":{}", j.name, self.get_json_val(field, struct_list));
                list.push(line);
            }
        }
        list.join("\n")
    }
    fn get_json_val(&self, field: &XField, struct_list: &HashMap<String, XST>) -> String {
        match field.stype {
            XType::XTypeStruct => {
                let sk = field.xtype.trim_start_matches('*').to_string();
                if let Some(v) = struct_list.get(&sk) {
                    return self.get_json(&v.fields, struct_list);
                }
            }
            XType::XTypeSlice => {
                let sk = field.xtype.replace("*", "").replace("[]", "");
                if let Some(v) = struct_list.get(&sk) {
                    return self.get_json(&v.fields, struct_list);
                } else {
                    return self.get_zero_val(&field.xtype);
                }
            }
            _ => {
                return self.get_zero_val(&field.xtype);
            }
        }
        "".to_string()
    }

    fn get_zero_val(&self, x_type: &str) -> String {
        match x_type {
            "bool" => "true".to_string(),
            x if x.contains("int") => "0".to_string(),
            x if x.contains("float") => "0.1".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn to_docs_item_fields(
        &self,
        fields: &HashMap<String, XField>,
        struct_list: &HashMap<String, XST>,
        prefix: String,
    ) -> Vec<docs::DocsItemField> {
        let mut _fields = self.sort_fields(fields);
        let mut request = Vec::new();
        for field in _fields.iter_mut() {
            let j = field.get_tag("json");
            let mut name = String::new();
            if let Some(j) = j {
                name = prefix.clone() + &j.name.replace(",omitempty", "");
            }
            let mut _type = field.xtype.clone();
            match field.stype {
                XType::XTypeStruct => {
                    let prefix = prefix.replace("[i].", "");
                    _type = "object".to_string();
                    request.push(docs::DocsItemField {
                        name: name.clone(),
                        type_: _type.clone(),
                        must: "Y".to_string(),
                        comment: field.comment.clone(),
                    });
                    let sk = field.xtype.trim_start_matches('*').to_string();
                    if let Some(v) = struct_list.get(&sk) {
                        let r = self.to_docs_item_fields(
                            &v.fields,
                            struct_list,
                            prefix.clone() + "&emsp;&emsp;",
                        );
                        request.extend(r);
                    }
                }
                XType::XTypeSlice => {
                    let prefix = prefix.replace("[i].", "");
                    _type = "array".to_string();
                    request.push(docs::DocsItemField {
                        name: name.clone(),
                        type_: _type.clone(),
                        must: "Y".to_string(),
                        comment: field.comment.clone(),
                    });
                    let sk = field.xtype.replace("*", "").replace("[]", "");
                    if let Some(v) = struct_list.get(&sk) {
                        let r = self.to_docs_item_fields(
                            &v.fields,
                            struct_list,
                            prefix.clone() + "&emsp;&emsp;[i].",
                        );
                        request.extend(r);
                    }
                }
                _ => {
                    request.push(docs::DocsItemField {
                        name: name.clone(),
                        type_: _type.clone(),
                        must: "Y".to_string(),
                        comment: field.comment.clone(),
                    });
                }
            }
        }
        request
    }
    fn sort_fields(&self, fields: &HashMap<String, XField>) -> Vec<XField> {
        let mut r = vec![XField::default(); fields.len()];
        for field in fields.values() {
            r[field.idx as usize] = field.clone();
        }
        r
    }
    fn gen_route_types(
        &self,
        module: &MetaNode,
        ef: &http_routes::HttpEntry,
    ) -> Result<Vec<GenerateData>> {
        let mut list = Vec::new();
        let mut has_struct_map = HashMap::new();
        let mut has_file_map = HashMap::new();
        if let Some(types) = module.find_by_name("types") {
            has_struct_map = types.go_struct_maps();
            for group in &ef.groups {
                let group_type = format!("io_{}.go", group.group);
                if let Some(file) = types.find_by_name(&group_type) {
                    has_file_map.insert(group_type, read_to_string(file.path)?);
                }
            }
        }
        for group in &ef.groups {
            let group_type = format!("io_{}.go", group.group);
            if let Some(file) = has_file_map.get(&group_type) {
                let mut _t = http_types::HandlerTypesAppend {
                    body: file.to_string(),
                    fun_list: Vec::new(),
                };
                for it in group.fun_list.iter() {
                    if !has_struct_map.contains_key(&it.req_name) {
                        _t.fun_list.push(it.clone());
                    }
                }
                let buf = _t.execute()?;
                list.push(GenerateData {
                    path: path_join(&[&module.path, "types", &group_type]),
                    gen_type: GenerateType::GenerateTypeMiman,
                    out_type: OutputType::OutputTypeGo,
                    content: buf,
                });
            } else {
                let _t = http_types::HandlerTypes {
                    entry_path: ef.entry_pkg_path.clone(),
                    entry: ef.entry_name.clone(),
                    group: group.group.clone(),
                    fun_list: group.fun_list.clone(),
                };
                let buf = _t.execute()?;
                list.push(GenerateData {
                    path: path_join(&[&module.path, "types", &group_type]),
                    gen_type: GenerateType::GenerateTypeMiman,
                    out_type: OutputType::OutputTypeGo,
                    content: buf,
                });
            }
        }
        Ok(list)
    }
    fn gen_route_handler(
        &self,
        module: &MetaNode,
        ef: &http_routes::HttpEntry,
    ) -> Result<Vec<GenerateData>> {
        let mut list = Vec::new();
        let mut has_func_map = HashMap::new();
        let mut has_file_map = HashMap::new();
        if let Some(handler) = module.find_by_name("handler") {
            has_func_map = handler.go_func_maps();
            for group in &ef.groups {
                let group_handler = format!("{}.go", group.group);
                if let Some(file) = handler.find_by_name(&group_handler) {
                    has_file_map.insert(group_handler, read_to_string(file.path)?);
                }
            }
        }
        for group in &ef.groups {
            let group_handler = format!("{}.go", group.group);
            if let Some(file) = has_file_map.get(&group_handler) {
                let mut _t = handler::HandlerFuncAppend {
                    body: file.to_string(),
                    fun_list: Vec::new(),
                };
                for it in group.fun_list.iter() {
                    if !has_func_map.contains_key(&it.fun_name) {
                        _t.fun_list.push(it.clone());
                    }
                }
                let buf = _t.execute()?;
                list.push(GenerateData {
                    path: path_join(&[&module.path, "handler", &group_handler]),
                    gen_type: GenerateType::GenerateTypeMiman,
                    out_type: OutputType::OutputTypeGo,
                    content: buf,
                });
            } else {
                let _t = handler::HandlerFunc {
                    entry_path: ef.entry_pkg_path.clone(),
                    entry: ef.entry_name.clone(),
                    group: group.group.clone(),
                    fun_list: group.fun_list.clone(),
                };
                let buf = _t.execute()?;
                list.push(GenerateData {
                    path: path_join(&[&module.path, "handler", &group_handler]),
                    gen_type: GenerateType::GenerateTypeMiman,
                    out_type: OutputType::OutputTypeGo,
                    content: buf,
                });
            }
        }
        Ok(list)
    }
    pub fn biz_entry_doc_parse(
        &self,
        entry_name: &str,
        doc: &str,
        socket: bool,
    ) -> Vec<http_routes::EntryGroup> {
        let handler_func_exp: Regex = Regex::new(r"(.+)\s+\[(\w+)]").unwrap();
        let handler_group_exp: Regex = Regex::new(r"#(\S+)\s+(\w+)").unwrap();
        let handler_middle_exp: Regex = Regex::new(r"@M\(([\w|,|\(|\)]+)\)").unwrap();
        let mut groups: Vec<http_routes::EntryGroup> = Vec::new();
        let lines: Vec<&str> = doc.split('\n').collect();

        for line in lines {
            let rg = find_string_sub_match(&handler_group_exp, line);
            if rg.len() == 3 {
                let mut group = http_routes::EntryGroup {
                    group: rg[2].to_string(),
                    group_ufirst: to_upper_first(&rg[2]),
                    group_name: rg[1].to_string(),
                    fun_list: Vec::new(),
                    gmiddlewares: Vec::new(),
                };

                if let Some(rm) = handler_middle_exp.captures(line) {
                    group.gmiddlewares = rm[1].split(',').map(|s| s.to_string()).collect();
                }
                groups.push(group);
                continue;
            }

            if groups.is_empty() {
                continue;
            }
            let groups_len = groups.len();
            let group = &mut groups[groups_len - 1];
            let r = find_string_sub_match(&handler_func_exp, line);
            if r.len() == 3 {
                let method = to_upper_first(&r[2]);
                let fun = to_upper_first(&group.group) + &method;
                let m = to_snake_case(&method).replace("_", "-");
                let group_uri_name = to_snake_case(&group.group).replace("_", "-");
                let mut item = http_routes::EntryFunItem {
                    fun_name: fun.clone(),
                    method,
                    fun_mark: r[1].to_string(),
                    req_name: fun.clone().to_string() + "Req",
                    resp_name: fun.to_string() + "Resp",
                    middlewares: Vec::new(),
                    uri: format!("/{}/{}/{}", entry_name, group_uri_name, m),
                    uri2: format!("/{}/{}", group_uri_name, m),
                };

                if socket {
                    item.uri = format!("{}.{}", group.group, m);
                }
                let rm = find_string_sub_match(&handler_middle_exp, line);
                if rm.len() == 2 {
                    item.middlewares = rm[1].split(',').map(|s| s.to_string()).collect();
                    if !group.gmiddlewares.is_empty() {
                        let mut middleware_map = HashSet::new();
                        let mut middlewares = Vec::new();

                        for middleware in &group.gmiddlewares {
                            middleware_map.insert(middleware.clone());
                            middlewares.push(middleware.clone());
                        }

                        for middleware in &item.middlewares {
                            if !middleware_map.contains(middleware) {
                                middlewares.push(middleware.clone());
                            }
                        }
                        item.middlewares = middlewares;
                    }
                } else {
                    item.middlewares = group.gmiddlewares.clone();
                }

                group.fun_list.push(item);
            }
        }

        groups
    }
    fn gen_app_types(
        &self,
        root: &str,
        pkg: &str,
        app: &MetaNode,
        module: &MetaNode,
        micro_apps: &Vec<MetaNode>,
    ) -> Result<Vec<GenerateData>> {
        let mut list = Vec::new();
        let app_rel_path = rel_path(root, &app.path);
        let app_pkg_path = format!("{}/{}", pkg, app_rel_path);
        let module_rel_path = rel_path(root, &module.path);
        let module_pkg_path = format!("{}/{}", pkg, module_rel_path);
        //micro types
        if let Some(main) = module.find_go_func("main") {
            if main.comment.len() > 0 {
                let use_micros = self.main_micro_parse(&main.comment);
                if use_micros.len() > 0 {
                    // 模块注册模式 显性控制需要引入的模块
                    // 读取micro配置
                    let mut use_mods = HashMap::new();
                    for use_micro in use_micros {
                        use_mods.insert(use_micro, true);
                    }
                    for micro_app in micro_apps.iter() {
                        if !use_mods.contains_key(&micro_app.name) {
                            continue;
                        }
                        let micro_package = format!("micro_{}", micro_app.name);
                        let micro_rel_path = rel_path(root, &micro_app.path);
                        let micro_pkg_path = format!("{}/{}", pkg, micro_rel_path);
                        let as_vecs: Vec<(String, String)> = vec![(
                            "types".to_string(),
                            format!("{}/types/{}", module_pkg_path, micro_package.clone()),
                        )];
                        let header_conv = header::HeaderWithAs {
                            package: "converter".to_string(),
                            imports: vec![
                                format!("{}/common/tools", pkg),
                                format!("{}/common/core/log", pkg),
                                format!("{}/common/tools/tool_time", pkg),
                                format!("{}/entity", micro_pkg_path),
                            ],
                            allow_edit: false,
                            as_vecs,
                        };
                        let header_micro = header::Header {
                            package: micro_package.clone(),
                            imports: vec!["time".to_string()],
                            allow_edit: true,
                        };
                        let mut bufc = header_conv.execute()?;
                        if let Some(entity) = micro_app.find_by_name("entity") {
                            let st_maps = entity.go_struct_maps();
                            let mut xst_maps: HashMap<String, Vec<XST>> = HashMap::new();
                            for (_, xst) in st_maps {
                                if !xst_maps.contains_key(&xst.file) {
                                    xst_maps.insert(xst.file.clone(), Vec::new());
                                }
                                xst_maps.get_mut(&xst.file).unwrap().push(xst);
                            }
                            let mut exist_maps = HashMap::new();
                            if let Some(types) = micro_app.find_by_name(&micro_package) {
                                exist_maps = types.go_struct_maps();
                            }
                            for (file, xst_list) in xst_maps {
                                let mut bufd = header_micro.execute()?;
                                let fname = path_name(&Path::new(&file));
                                for xst in xst_list {
                                    let xst_default = &mut XST::default();
                                    let old_xst =
                                        exist_maps.get_mut(&xst.name).unwrap_or(xst_default);
                                    let name_mark =
                                        format!("Micro{}", to_upper_first(&micro_app.name));
                                    let (_b, _bc) =
                                        self._types(&xst, old_xst, "json", &name_mark)?;
                                    bufd += &_b;
                                    bufc += &_bc;
                                }
                                list.push(GenerateData {
                                    path: path_join(&[
                                        &module.path,
                                        "types",
                                        &micro_package,
                                        &format!("entity_{}", fname),
                                    ]),
                                    gen_type: GenerateType::GenerateTypeMiman,
                                    out_type: OutputType::OutputTypeGo,
                                    content: bufd,
                                })
                            }
                            list.push(GenerateData {
                                path: path_join(&[
                                    &module.path,
                                    "converter",
                                    &format!("{}_converter_gen.go", micro_package),
                                ]),
                                gen_type: GenerateType::GenerateTypeMiman,
                                out_type: OutputType::OutputTypeGo,
                                content: bufc,
                            })
                        }
                    }
                }
            }
        }
        //business types
        if let Some(entity) = app.find_by_name("entity") {
            let xst_list = entity.go_struct_list();
            let mut xst_maps: HashMap<String, Vec<XST>> = HashMap::new();
            for xst in xst_list {
                if !xst_maps.contains_key(&xst.file) {
                    xst_maps.insert(xst.file.clone(), Vec::new());
                }
                xst_maps.get_mut(&xst.file).unwrap().push(xst);
            }
            let mut exist_maps = HashMap::new();
            if let Some(types) = module.find_by_name("types") {
                exist_maps = types.go_struct_maps();
            }
            let header_conv = header::Header {
                package: "converter".to_string(),
                imports: vec![
                    format!("{}/common/tools", pkg),
                    format!("{}/common/core/log", pkg),
                    format!("{}/common/tools/tool_time", pkg),
                    format!("{}/entity", app_pkg_path),
                    format!("{}/types", module_pkg_path),
                ],
                allow_edit: false,
            };
            let header_types = header::Header {
                package: "types".to_string(),
                imports: vec!["time".to_string()],
                allow_edit: true,
            };
            let mut bufc = header_conv.execute()?;
            for (file, xst_list) in xst_maps {
                let mut bufd = header_types.execute()?;
                let fname = path_name(&Path::new(&file));
                for xst in xst_list {
                    let xst_default = &mut XST::default();
                    let old_xst = exist_maps.get_mut(&xst.name).unwrap_or(xst_default);
                    let (_b, _bc) = self._types(&xst, old_xst, "json", "")?;
                    bufd += &_b;
                    bufc += &_bc;
                }
                list.push(GenerateData {
                    path: path_join(&[&module.path, "types", &format!("entity_{}", fname)]),
                    gen_type: GenerateType::GenerateTypeMiman,
                    out_type: OutputType::OutputTypeGo,
                    content: bufd,
                })
            }
            list.push(GenerateData {
                path: path_join(&[&module.path, "converter", "entity_converter_gen.go"]),
                gen_type: GenerateType::GenerateTypeMiman,
                out_type: OutputType::OutputTypeGo,
                content: bufc,
            })
        }
        Ok(list)
    }
    fn _types(
        &self,
        xst: &XST,
        old_xst: &mut XST,
        tag_name: &str,
        name_mark: &str,
    ) -> Result<(String, String)> {
        let mut gio = types::IO {
            name: xst.name.clone(),
            fields: Vec::new(),
        };
        let mut field_list = Vec::new();
        for (_, field) in &xst.fields {
            field_list.push(field.clone());
        }
        field_list.sort_by(|a, b| a.idx.cmp(&b.idx));

        for field in &mut field_list {
            let tag_json = field.get_tag("json");
            let tag_io = field.get_tag(tag_name);
            let mut tags = String::new();

            if tag_json.is_none() {
                continue;
            }
            if let Some(tag_json) = &tag_json {
                tags = format!("`json:\"{}\"`", tag_json.name);
                if tag_json.name == "-" {
                    tags = String::new();
                }
            }
            if let Some(tag_io) = tag_io {
                if tag_io.txt == "-" {
                    continue;
                }
                if !tag_io.txt.is_empty() {
                    tag_json.unwrap().name = tag_io.name.clone();
                }
            }

            let mut type2 = String::new();
            let mut type2_entity = false;

            let mut ftype = field.xtype.clone();
            match field.stype {
                XType::XTypeStruct => {
                    type2 = field.xtype.replace("*", "");
                    if field.xtype.contains("time.Time") {
                        field.stype = XType::XTypeTime;
                        ftype = "string".to_string();
                    }
                }
                XType::XTypeSlice => {
                    type2 = field.xtype.replace("[]", "");
                    type2 = type2.replace("*", "");
                    if is_first_uppercase(&type2) {
                        type2_entity = true;
                    }
                }
                _ => {}
            }

            if let Some(old_field) = old_xst.fields.get(&field.name) {
                let tag_json2 = old_field.get_tag("json");
                if let Some(tag_json2) = tag_json2 {
                    if tag_json2.name == "-" {
                        gio.fields.push(types::IoField {
                            name: field.name.clone(),
                            type_: ftype.clone(),
                            type2: type2.clone(),
                            type2_entity,
                            stype: field.stype.clone() as i32,
                            tag: "`json:\"-\"`".to_string(),
                            comment: field.comment.clone(),
                            hidden: false,
                        });
                        continue;
                    }
                }
            }

            gio.fields.push(types::IoField {
                name: field.name.clone(),
                type_: ftype.clone(),
                type2: type2.clone(),
                type2_entity,
                stype: field.stype.clone() as i32,
                tag: tags,
                comment: field.comment.clone(),
                hidden: false,
            });
        }

        if gio.fields.is_empty() {
            return Ok(("".to_string(), "".to_string()));
        }

        let conv_buf = self._io_conv(&mut gio, name_mark)?;
        // 自定义字段
        for (_, field) in old_xst.fields.iter_mut() {
            if !xst.fields.contains_key(&field.name) {
                let tag_json = field.get_tag("json");
                let tag_io = field.get_tag(tag_name);
                let mut tags = String::new();

                if tag_json.is_none() {
                    continue;
                }
                if let Some(tag_json) = &tag_json {
                    tags = format!("`json:\"{}\"`", tag_json.name);
                    if tag_json.name == "-" {
                        tags = String::new();
                    }
                }
                if let Some(tag_io) = tag_io {
                    if tag_io.txt == "-" {
                        continue;
                    }
                    if !tag_io.txt.is_empty() {
                        tag_json.unwrap().name = tag_io.name.clone();
                    }
                }

                let mut type2 = String::new();
                let mut type2_entity = false;

                let mut ftype = field.xtype.clone();
                match field.stype {
                    XType::XTypeStruct => {
                        type2 = field.xtype.replace("*", "");
                        if field.xtype.contains("time.Time") {
                            field.stype = XType::XTypeTime;
                            ftype = "string".to_string();
                        }
                    }
                    XType::XTypeSlice => {
                        type2 = field.xtype.replace("[]", "");
                        type2 = type2.replace("*", "");
                        if is_first_uppercase(&type2) {
                            type2_entity = true;
                        }
                    }
                    _ => {}
                }

                gio.fields.push(types::IoField {
                    name: field.name.clone(),
                    type_: ftype,
                    type2,
                    type2_entity,
                    stype: field.stype.clone() as i32,
                    tag: tags,
                    comment: field.comment.clone(),
                    hidden: false,
                });
            }
        }

        let buf = gio.execute()?;
        Ok((buf, conv_buf))
    }

    fn _io_conv(&self, gio: &mut types::IO, name_mark: &str) -> Result<String> {
        for item in &mut gio.fields {
            if item.name.is_empty() {
                item.name = item.type2.clone();
            }
        }
        let conv_gen = types::IoConv {
            name: gio.name.clone(),
            name_mark: name_mark.to_string(),
            fields: gio.fields.clone(),
            ..Default::default()
        };
        let buf = conv_gen.execute()?;
        Ok(buf)
    }
    pub fn main_micro_parse(&self, doc: &str) -> Vec<String> {
        let mut list: Vec<String> = Vec::new();
        let micro_exp = Regex::new(r"@MICRO\[([\w|,]+)]").unwrap();
        let r = find_string_sub_match(&micro_exp, doc);
        if r.len() > 1 {
            list = r[1].split(",").map(String::from).collect();
        }
        list
    }
    fn gen_micro_func(
        &self,
        root: &str,
        pkg: &str,
        data: &MetaNode,
    ) -> Result<(bool, Vec<GenerateData>)> {
        let mut need_provide = false;
        let mut list = Vec::new();
        let rel_path = rel_path(root, &data.path);
        let pkg_path = format!("{}/{}", pkg, rel_path);
        let entry = data.find_by_name("entry.go");
        if let Some(ent) = entry {
            let func_maps = ent.go_func_maps();
            if let Some(mt) = func_maps.get("_gen") {
                let items = self.micro_entry_doc_parse(&mt.comment);
                if items.len() > 0 {
                    need_provide = true;
                }
                let micro_entry = micro_entry::MicroEntry {
                    project: pkg.to_string(),
                    app_name: data.name.clone(),
                    app_name_uf: to_upper_first(&data.name),
                    app_pkg_path: pkg_path,
                    fun_list: items.clone(),
                };
                let (bufe, bufs) = (
                    micro_entry.execute(micro_entry::MICRO_ENTRY_TPL)?,
                    micro_entry.execute(micro_entry::MICRO_SERVICE_TPL)?,
                );
                let pname = data.name.clone() + "_gen.go";
                list.append(&mut vec![
                    GenerateData {
                        path: path_join(&[&root, "provider", &pname]),
                        gen_type: self.generate_type(),
                        out_type: OutputType::OutputTypeGo,
                        content: bufe,
                    },
                    GenerateData {
                        path: path_join(&[&data.path, "service_gen.go"]),
                        gen_type: self.generate_type(),
                        out_type: OutputType::OutputTypeGo,
                        content: bufs,
                    },
                ]);
                let mut func_list = self.micro_func_io(root, pkg, data, &items)?;
                list.append(&mut func_list);
                let mut service_list = self.micro_service(root, pkg, data, &items)?;
                list.append(&mut service_list);
            }
        }
        Ok((need_provide, list))
    }
    fn micro_service(
        &self,
        root: &str,
        pkg: &str,
        data: &MetaNode,
        items: &Vec<micro_entry::MicroFunItem>,
    ) -> Result<Vec<GenerateData>> {
        let mut list = Vec::new();
        let rel_path = rel_path(root, &data.path);
        let pkg_path = format!("{}/{}", pkg, rel_path);
        let mut items_map: HashMap<String, Vec<micro_entry::MicroFunItem>> = HashMap::new();
        for item in items {
            if !items_map.contains_key(&item.service) {
                items_map.insert(item.service.clone(), Vec::new());
            }
            items_map.get_mut(&item.service).unwrap().push(item.clone());
        }
        let service_dir = data.find_by_name("service");
        for (service, func_items) in items_map {
            let gen_name = format!("{}.go", service.to_lowercase());
            match &service_dir {
                Some(dir) => match dir.find_by_name(&gen_name) {
                    Some(file) => {
                        let xst_maps = dir.go_struct_maps();
                        let _buf = read_to_string(file.path)?;
                        let mut tpl = micro_service::MicroServiceAppend {
                            body: _buf,
                            fun_list: Vec::new(),
                            app_name: data.name.clone(),
                        };
                        if let Some(xst) = xst_maps.get(&service) {
                            for it in &func_items {
                                if let None = xst.methods.get(&it.method) {
                                    tpl.fun_list.push(it.clone());
                                }
                            }
                        }
                        if tpl.fun_list.len() > 0 {
                            list.push(GenerateData {
                                path: path_join(&[&data.path, "service", &gen_name]),
                                gen_type: self.generate_type(),
                                out_type: OutputType::OutputTypeGo,
                                content: tpl.execute()?,
                            });
                        }
                    }
                    None => {
                        let tpl = micro_service::MicroServiceFunc {
                            app_name: data.name.clone(),
                            app_pkg_name: pkg_path.clone(),
                            service,
                            fun_list: func_items,
                        };
                        if tpl.fun_list.len() > 0 {
                            list.push(GenerateData {
                                path: path_join(&[&data.path, "service", &gen_name]),
                                gen_type: self.generate_type(),
                                out_type: OutputType::OutputTypeGo,
                                content: tpl.execute()?,
                            });
                        }
                    }
                },
                None => {
                    let tpl = micro_service::MicroServiceFunc {
                        app_name: data.name.clone(),
                        app_pkg_name: pkg_path.clone(),
                        service,
                        fun_list: func_items,
                    };
                    if tpl.fun_list.len() > 0 {
                        list.push(GenerateData {
                            path: path_join(&[&data.path, "service", &gen_name]),
                            gen_type: self.generate_type(),
                            out_type: OutputType::OutputTypeGo,
                            content: tpl.execute()?,
                        });
                    }
                }
            }
        }
        Ok(list)
    }
    fn micro_func_io(
        &self,
        root: &str,
        pkg: &str,
        data: &MetaNode,
        items: &Vec<micro_entry::MicroFunItem>,
    ) -> Result<Vec<GenerateData>> {
        let mut list = Vec::new();
        let rel_path = rel_path(root, &data.path);
        let pkg_path = format!("{}/{}", pkg, rel_path);
        let type_dir = format!("types_{}", data.name);
        match data.find_by_name(&type_dir) {
            Some(dir) => match dir.find_by_name("types.go") {
                Some(file) => {
                    let xst_maps = dir.go_struct_maps();
                    let _buf = read_to_string(file.path)?;
                    let mut tpl = micro_types::MicroTypesAppend {
                        body: _buf,
                        fun_list: Vec::new(),
                    };
                    for it in items {
                        if let None = xst_maps.get(&it.req_name) {
                            tpl.fun_list.push(it.clone());
                        }
                    }
                    if tpl.fun_list.len() > 0 {
                        list.push(GenerateData {
                            path: path_join(&[&data.path, &type_dir, "types.go"]),
                            gen_type: self.generate_type(),
                            out_type: OutputType::OutputTypeGo,
                            content: tpl.execute()?,
                        });
                    }
                }
                None => {
                    let tpl = micro_types::MicroTypes {
                        app_name: data.name.clone(),
                        app_pkg_path: pkg_path,
                        fun_list: items.clone(),
                    };
                    if tpl.fun_list.len() > 0 {
                        list.push(GenerateData {
                            path: path_join(&[&data.path, &type_dir, "types.go"]),
                            gen_type: self.generate_type(),
                            out_type: OutputType::OutputTypeGo,
                            content: tpl.execute()?,
                        });
                    }
                }
            },
            None => {
                let tpl = micro_types::MicroTypes {
                    app_name: data.name.clone(),
                    app_pkg_path: pkg_path,
                    fun_list: items.clone(),
                };
                if tpl.fun_list.len() > 0 {
                    list.push(GenerateData {
                        path: path_join(&[&data.path, &type_dir, "types.go"]),
                        gen_type: self.generate_type(),
                        out_type: OutputType::OutputTypeGo,
                        content: tpl.execute()?,
                    });
                }
            }
        }
        Ok(list)
    }
    fn micro_entry_doc_parse(&self, doc: &str) -> Vec<micro_entry::MicroFunItem> {
        let mut list = Vec::new();
        let micro_fun_exp = Regex::new(r"(\w+)\s+(.+)\s+\[([\w|.]+)]").unwrap();
        let lines = doc.split("\n");
        for line in lines {
            let r = find_string_sub_match(&micro_fun_exp, line);
            if r.len() == 4 {
                let services: Vec<&str> = r[3].split(".").collect();
                let (mut service, mut method) = ("".to_string(), "".to_string());
                if services.len() > 0 {
                    service = services[0].to_string();
                }
                if services.len() > 1 {
                    method = services[1].to_string();
                }
                let func_name = r.get(1).unwrap().to_string();
                let fun_mark = r.get(2).unwrap().to_string();
                let item = micro_entry::MicroFunItem {
                    service,
                    method,
                    fun_name: func_name.clone(),
                    fun_mark,
                    req_name: format!("{}Req", func_name),
                    resp_name: format!("{}Resp", func_name),
                };
                list.push(item);
            }
        }
        list
    }
    fn gen_gi(&self, data: &MetaNode) -> Result<GenerateData> {
        let xst_list = data.go_struct_list();
        let new_func_map = data.go_new_func_maps();
        let mut gdi = gi_def::Gi {
            pkg: data.name.clone(),
            list: Vec::new(),
        };
        for xst in xst_list {
            if xst.gi {
                let mut it = gi_def::GItem {
                    name: xst.name.clone(),
                    name_val: to_lower_first(&xst.name),
                    new_returns_len: 0,
                };
                if let Some(nmth) = new_func_map.get(&xst.name) {
                    it.new_returns_len = nmth.results.len();
                }
                gdi.list.push(it);
            }
        }
        let buf = gdi.execute()?;
        let gen_data = GenerateData {
            path: path_join(&[&data.path, "gi_gen.go"]),
            gen_type: self.generate_type(),
            out_type: OutputType::OutputTypeGo,
            content: buf,
        };
        Ok(gen_data)
    }
    fn gen_do_next(&self, root: &str, pkg: &str, data: &MetaNode) -> Result<Vec<GenerateData>> {
        let path_parent = path_parent(&data.path);
        let rel_path = rel_path(root, &path_parent);
        let pkg_path = format!("{}/{}", pkg, rel_path);
        let xst_list = data.go_struct_list();
        let header_do = type_def::EntityTypeImport {
            project_name: pkg.to_string(),
            package_name: "do".to_string(),
        };
        let mut buf_do = header_do.execute()?;
        let header_dao = header::Header {
            package: "dao".to_string(),
            imports: vec![
                "github.com/pkg/errors".to_string(),
                "gorm.io/gorm".to_string(),
                format!("{}/do", pkg_path),
            ],
            allow_edit: false,
        };
        let mut buf_dao = header_dao.execute()?;
        for xst in xst_list {
            let _buf_do = self.type_def(pkg, &xst)?;
            buf_do += &_buf_do;
            let _buf_dao = self.dao_def(&xst)?;
            buf_dao += &_buf_dao;
        }

        let list = vec![
            GenerateData {
                path: path_join(&[&path_parent, "do", "type_def_code_gen.go"]),
                gen_type: self.generate_type(),
                out_type: OutputType::OutputTypeGo,
                content: buf_do,
            },
            GenerateData {
                path: path_join(&[&path_parent, "dao", "dao_gen.go"]),
                gen_type: self.generate_type(),
                out_type: OutputType::OutputTypeGo,
                content: buf_dao,
            },
        ];
        Ok(list)
    }
    fn gen_repos(&self, root: &str, pkg: &str, data: &MetaNode) -> Result<Vec<GenerateData>> {
        let mut list = Vec::new();
        let path_parent = path_parent(&data.path);
        let rel_path = rel_path(root, &path_parent);
        let pkg_path = format!("{}/{}", pkg, rel_path);
        let mut entity_list: Vec<String> = Vec::new();
        let mut has_id_map: HashMap<String, bool> = HashMap::new();
        let xst_list = data.go_struct_list();
        for xst in xst_list {
            for (_, field) in &xst.fields {
                let tag = field.get_tag("db");
                if let Some(tag) = tag {
                    if tag.txt != "-" {
                        entity_list.push(xst.name.clone());
                        break;
                    }
                }
            }
            for (_, field) in xst.fields {
                if field.name == "ID" {
                    has_id_map.insert(xst.name.clone(), true);
                    break;
                }
            }
        }
        for entity in entity_list {
            let table_name = to_snake_case(&entity);
            let tpl = repo_def::Repo {
                project_name: pkg.to_string(),
                app_pkg_path: pkg_path.clone(),
                entity_name: entity.clone(),
                table_name: table_name.clone(),
                has_id: has_id_map.get(&entity).unwrap_or(&false).to_owned(),
            };
            let (buf_repo, buf_dbal) = (tpl.execute()?, tpl.execute_impl()?);
            list.append(&mut vec![
                GenerateData {
                    path: path_join(&[&path_parent, "repo", &format!("{}_repo.go", table_name)]),
                    gen_type: self.generate_type(),
                    out_type: OutputType::OutputTypeGo,
                    content: buf_repo,
                },
                GenerateData {
                    path: path_join(&[
                        &path_parent,
                        "repo",
                        "dbal",
                        &format!("{}_dbal.go", table_name),
                    ]),
                    gen_type: self.generate_type(),
                    out_type: OutputType::OutputTypeGo,
                    content: buf_dbal,
                },
            ]);
        }
        Ok(list)
    }
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
            buf += &self.type_def(pkg, &xst)?;
        }
        gen_data.content = buf;
        Ok(gen_data)
    }

    fn dao_def(&self, xst: &XST) -> Result<String> {
        let (mut pk_name, mut pk_type, mut pk_col) =
            ("".to_string(), "".to_string(), "".to_string());
        for (_, field) in &xst.fields {
            let tag = field.get_tag("gorm");
            if let Some(desc) = tag {
                if desc.txt.contains("primaryKey") {
                    pk_name = field.name.clone();
                    pk_type = field.xtype.clone();
                    pk_col = desc.name;
                }
            }
        }
        let dao_name = format!("{}Dao", xst.name.trim_end_matches("Do"));
        let entity_list_name = format!("{}List", xst.name);
        let table_name = format!("do.TableName{}", xst.name);
        let dao = dao_def::Dao {
            entity_name: xst.name.clone(),
            dao_name,
            entity_list_name,
            table_name,
            pk_name,
            pk_type,
            pk_col,
        };
        let buf = dao.execute()?;
        Ok(buf)
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
    fn type_def(&self, pkg: &str, xst: &XST) -> Result<String> {
        let mut tpl = type_def::EntityTypeMap {
            project_name: pkg.to_string(),
            entity_name: xst.name.clone(),
            entity_list_name: format!("{}List", xst.name),
            fields: Vec::new(),
            has_creator: false,
            creator_name: "".to_string(),
        };
        let mut field_list = Vec::new();
        for (_, field) in &xst.fields {
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
