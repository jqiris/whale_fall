use std::collections::HashMap;

use gosyn::*;
pub struct XArg {
    pub name: String,
    pub xtype: String,
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

pub enum ParserType {
    ParserTypeGM,
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
pub enum MetaData {
    Doc(String),
    Go(MetaGo),
}
pub struct MetaNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub child: Vec<MetaNode>,
    pub data: MetaData,
}

pub enum ProcessType {
    ProcessTypeMiman,
}

pub struct ProcessData {
    pub lists: HashMap<String, Vec<MetaNode>>,
    pub maps: HashMap<String, HashMap<String, MetaNode>>,
}

pub enum GenerateType {
    GenerateTypeMiman,
}

pub struct GenerateData {}

pub enum OutputType {
    OutputTypeGo,
    OutputTypeMd,
}
