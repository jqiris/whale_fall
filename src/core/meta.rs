use crate::common::go::{MetaGo, XST};
use std::collections::HashMap;

pub enum ParserType {
    ParserTypeGM,
}
#[derive(Clone)]
pub enum MetaData {
    Md(String),
    Go(MetaGo),
}
#[derive(Clone)]
pub struct MetaNode {
    pub name: String,           //名字
    pub path: String,           //路径
    pub is_dir: bool,           //是否是目录
    pub childs: Vec<MetaNode>,  //子节点
    pub data: Option<MetaData>, //数据
}
impl MetaNode {
    pub fn find_by_name(&self, name: &str) -> Option<MetaNode> {
        if self.name == name {
            return Some(self.clone());
        }
        for child in self.childs.iter() {
            if let Some(meta) = child.find_by_name(name) {
                return Some(meta);
            }
        }
        None
    }
    pub fn find_list_by_name(&self, name: &str) -> Vec<MetaNode> {
        let mut list = Vec::new();
        if self.name == name {
            list.push(self.clone());
        }
        for child in self.childs.iter() {
            list.append(&mut child.find_list_by_name(name));
        }
        list
    }

    pub fn go_struct_list(&self) -> Vec<XST> {
        let mut list = Vec::new();
        if let Some(data) = &self.data {
            if let MetaData::Go(go) = data {
                list.append(&mut go.st_list.values().cloned().collect());
            }
        }
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }
}

pub enum ProcessType {
    ProcessTypeMiman,
}

pub struct ProcessData {
    pub lists: HashMap<String, Vec<MetaNode>>,
    pub maps: HashMap<String, MetaNode>,
}

pub enum GenerateType {
    GenerateTypeMiman,
}

pub struct GenerateData {
    pub path: String,
    pub gen_type: GenerateType,
    pub out_type: OutputType,
    pub content: String,
}

pub enum OutputType {
    OutputTypeGo,
    OutputTypeMd,
}
