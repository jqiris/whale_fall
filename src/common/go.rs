use std::collections::HashMap;

use gosyn::ast::{self, *};
use regex::Regex;

use crate::core::meta::*;

use super::{
    file::path_str,
    str::{find_string_sub_match, is_first_uppercase},
};
pub enum XType {
    XTypeNone,
    XTypeBasic,
    XTypeStruct,
    XTypeSlice,
    XTypeMap,
    XTypeTime,
}

pub struct XArg {
    pub name: String,
    pub xtype: String,
}

impl Default for XArg {
    fn default() -> Self {
        XArg {
            name: "".to_string(),
            xtype: "".to_string(),
        }
    }
}

pub struct XMethod {
    pub impl_name: String,
    pub name: String,
    pub params: Vec<XArg>,
    pub results: Vec<XArg>,
    pub comment: String,
    pub sort: i32,
    pub http_rule: String,
    pub http_method: String,
}

pub struct XField {
    pub name: String,
    pub xtype: String,
    pub stype: i32,
    pub idx: i32,
    pub tag: String,
    pub comment: String,
}

pub struct XST {
    pub gi_name: String,
    pub gi: bool,
    pub impl_inf: String,
    pub imports: Vec<String>,
    pub file: String,
    pub name: String,
    pub short_name: String,
    pub mpoint: bool,
    pub cst: Vec<String>,
    pub methods: Vec<XMethod>,
    pub fields: HashMap<String, XField>,
}
pub struct INF {
    pub name: String,
    pub file: String,
    pub imports: Vec<String>,
    pub methods: HashMap<String, XMethod>,
}

pub struct MetaGo {
    pub ast_file: Option<ast::File>,
    pub inf_list: HashMap<String, INF>,      //interface list
    pub st_list: HashMap<String, XST>,       //struct list
    pub ot_list: HashMap<String, XST>,       //other type list
    pub const_list: HashMap<String, String>, //const list
    pub bind_func_list: HashMap<String, HashMap<String, XMethod>>, //bind func
    pub func_list: HashMap<String, XMethod>, //func
    pub new_func_list: HashMap<String, XMethod>, //new func
}

pub fn go_map_type_str(arg: MapType) -> String {
    let (mut key, mut value) = ("".to_string(), "".to_string());
    match *arg.key {
        Expression::Ident(x) => {
            key = x.name;
        }
        _ => {}
    }
    let (value, _) = go_type_str(*arg.val);
    format!("map[{}]{}", key, value)
}

pub fn go_slice_type_str(arg: ArrayType) -> String {
    let (value, _) = go_type_str(*arg.typ);
    format!("[]{}", value)
}

pub fn go_type_str(arg: Expression) -> (String, XType) {
    match arg {
        Expression::Selector(x) => (format!("{:?}.{}", x.x, x.sel.name), XType::XTypeStruct),
        Expression::Star(x) => match *x.right {
            Expression::Selector(_type) => (
                format!("{:?}.{}", _type.x, _type.sel.name),
                XType::XTypeStruct,
            ),
            Expression::Ident(_type) => {
                if is_first_uppercase(_type.name.clone()) {
                    return (format!("*{}", _type.name), XType::XTypeStruct);
                }
                (format!("*{}", _type.name), XType::XTypeBasic)
            }
            _ => ("".to_string(), XType::XTypeNone),
        },
        Expression::Ident(x) => {
            if is_first_uppercase(x.name.clone()) {
                return (x.name, XType::XTypeStruct);
            }
            (x.name, XType::XTypeBasic)
        }
        Expression::TypeMap(x) => (go_map_type_str(x), XType::XTypeMap),
        Expression::TypeArray(x) => (go_slice_type_str(x), XType::XTypeSlice),
        Expression::TypeInterface(_) => ("interface{}".to_string(), XType::XTypeBasic),
        _ => ("".to_string(), XType::XTypeBasic),
    }
}

pub fn go_func_args(xtype: FuncType) -> (Vec<XArg>, Vec<XArg>) {
    let mut params = Vec::new();
    let mut results = Vec::new();
    for (idx, param) in xtype.params.list.iter().enumerate() {
        let mut arg = XArg::default();
        (arg.xtype, _) = go_type_str(param.typ.clone());
        if param.name.len() > 0 {
            arg.name = param.name[0].name.clone();
        } else {
            if arg.xtype.contains("Context") {
                arg.name = "ctx".to_string();
            } else if arg.xtype.contains("Request") {
                arg.name = "req".to_string();
            } else {
                arg.name = format!("arg{}", idx);
            }
        }
        params.push(arg);
    }
    for ret in xtype.result.list {
        let mut arg = XArg::default();
        (arg.xtype, _) = go_type_str(ret.typ);
        if ret.name.len() > 0 {
            arg.name = ret.name[0].name.clone();
        }
        results.push(arg);
    }
    (params, results)
}

pub fn go_interface_func(xtype: InterfaceType) -> HashMap<String, XMethod> {
    let mut methods = HashMap::new();
    for (idx, mt) in xtype.methods.list.iter().enumerate() {
        let comment: String = mt.comments.iter().map(|x| x.text.clone()).collect();
        let mt_type = mt.typ.clone();
        match mt_type {
            gosyn::ast::Expression::TypeInterface(data) => {
                let mths = go_interface_func(data);
                for (k, v) in mths {
                    if methods.contains_key(k.as_str()) {
                        continue;
                    }
                    methods.insert(k, v);
                }
            }
            gosyn::ast::Expression::TypeFunction(data) => {
                let (params, results) = go_func_args(data);
                let method = XMethod {
                    name: mt.name[0].name.clone(),
                    impl_name: "impl".to_string(),
                    params,
                    results,
                    comment,
                    sort: idx as i32,
                    http_rule: "".to_string(),
                    http_method: "".to_string(),
                };
                methods.insert(method.name.clone(), method);
            }
            _ => {}
        }
    }
    methods
}

impl From<ast::File> for MetaGo {
    fn from(ast_file: ast::File) -> Self {
        let mut metaGo = MetaGo {
            ast_file: Some(ast_file.clone()),
            inf_list: HashMap::new(),
            st_list: HashMap::new(),
            ot_list: HashMap::new(),
            const_list: HashMap::new(),
            bind_func_list: HashMap::new(),
            func_list: HashMap::new(),
            new_func_list: HashMap::new(),
        };
        let re_impl = Regex::new(r"@IMPL\[([\w|.]+)]").unwrap();
        let re_di = Regex::new(r"@DI\[([\w|.]+)]").unwrap();
        let mut imports = Vec::new();
        for import in ast_file.imports {
            imports.push(import.path.value);
        }
        for decl in ast_file.decl {
            match decl {
                ast::Declaration::Function(_) => todo!(),
                ast::Declaration::Type(x) => {
                    let (mut used, mut impl_inf, mut gi_name, mut gi) = (true, "", "", false);
                    if x.docs.len() > 0 {
                        for comment in x.docs {
                            if comment.text.contains("@IGNORE") {
                                used = false;
                            }
                            if comment.text.contains("@IMPL[") {
                                let rs = find_string_sub_match(&re_impl, &comment.text);
                                if rs.len() > 1 {
                                    impl_inf = rs[1].as_str();
                                }
                            }
                            if comment.text.contains("@GI") {
                                let rs = find_string_sub_match(&re_di, &comment.text);
                                if rs.len() > 1 {
                                    gi_name = rs[1].as_str();
                                    gi = true;
                                }
                            }
                        }
                    }
                    for spec in x.specs {
                        let name = spec.name.name;
                        match spec.typ {
                            ast::Expression::TypeStruct(xt) => {}
                            ast::Expression::TypeInterface(xt) => {
                                if !used {
                                    continue;
                                }
                                let inf = INF {
                                    name: name.clone(),
                                    file: path_str(&ast_file.path),
                                    imports: imports.clone(),
                                    methods: go_interface_func(xt),
                                };
                                metaGo.inf_list.insert(name, inf);
                            }
                            _ => {}
                        }
                    }
                }
                ast::Declaration::Const(_) => todo!(),
                ast::Declaration::Variable(_) => todo!(),
            }
        }
        metaGo
    }
}
