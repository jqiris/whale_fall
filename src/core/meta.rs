use std::{collections::HashMap, path::Path};

use gosyn::*;
use regex::Regex;

use crate::common::{file::path_str, go::MetaGo, str::find_string_sub_match};

pub enum ParserType {
    ParserTypeGM,
}
pub enum MetaData {
    Doc(String),
    Go(MetaGo),
}
pub struct MetaNode {
    pub name: String,           //名字
    pub path: String,           //路径
    pub is_dir: bool,           //是否是目录
    pub childs: Vec<MetaNode>,  //子节点
    pub data: Option<MetaData>, //数据
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
