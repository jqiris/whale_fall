use crate::asts::go::{MetaGo, XMethod, INF, XST};
use crate::common::str::in_slice;
use std::collections::HashMap;

pub enum ParserType {
    ParserTypeGM,
}
#[derive(Debug, Clone)]
pub enum ExtraData {
    MetaList(Vec<MetaNode>),
}
#[derive(Debug, Clone)]
pub enum MetaData {
    Md(String),
    Go(MetaGo),
}
#[derive(Debug, Clone)]
pub struct MetaNode {
    pub name: String,                                   //名字
    pub path: String,                                   //路径
    pub is_dir: bool,                                   //是否是目录
    pub childs: Vec<MetaNode>,                          //子节点
    pub data: Option<MetaData>,                         //数据
    pub extra_data: Option<HashMap<String, ExtraData>>, //额外数据
}
impl MetaNode {
    pub fn get_dir_childs(&self) -> Vec<MetaNode> {
        let mut list = Vec::new();
        for child in self.childs.iter() {
            if child.is_dir {
                list.push(child.clone());
            }
        }
        list
    }
    pub fn child_dir_exist(&self, name: &str) -> bool {
        for child in self.childs.iter() {
            if child.is_dir && child.name == name {
                return true;
            }
        }
        false
    }

    pub fn find_go_func(&self, name: &str) -> Option<XMethod> {
        if let Some(data) = &self.data {
            if let MetaData::Go(go) = data {
                for (_, func) in go.func_list.iter() {
                    if func.name == name {
                        return Some(func.clone());
                    }
                }
            }
            for child in &self.childs {
                if let Some(func) = child.find_go_func(name) {
                    return Some(func);
                }
            }
        }
        None
    }
    pub fn find_go_inf(&self, name: &str) -> Option<INF> {
        if let Some(data) = &self.data {
            if let MetaData::Go(go) = data {
                for (_, inf) in go.inf_list.iter() {
                    if inf.name == name {
                        return Some(inf.clone());
                    }
                }
            }
            for child in &self.childs {
                if let Some(inf) = child.find_go_inf(name) {
                    return Some(inf);
                }
            }
        }
        None
    }
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
    pub fn find_gi_list(&self, excludes: &[&str]) -> Vec<MetaNode> {
        let mut list = Vec::new();
        if !self.is_dir {
            return list;
        }
        if in_slice(excludes, &self.name) {
            return list;
        }
        let xst_list = self.go_struct_list();
        for xst in xst_list.iter() {
            if xst.gi {
                list.push(self.clone());
                break;
            }
        }
        for child in self.childs.iter() {
            if !child.is_dir {
                continue;
            }
            if in_slice(excludes, &child.name) {
                continue;
            }
            let mut child_list = child.find_gi_list(excludes);
            if child_list.len() > 0 {
                list.append(&mut child_list);
            }
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
    pub fn go_struct_maps(&self) -> HashMap<String, XST> {
        let mut maps = HashMap::new();
        if let Some(data) = &self.data {
            if let MetaData::Go(go) = data {
                for (k, v) in go.st_list.iter() {
                    maps.insert(k.clone(), v.clone());
                }
            }
        }
        maps
    }
    pub fn go_func_maps(&self) -> HashMap<String, XMethod> {
        let mut maps = HashMap::new();
        if let Some(data) = &self.data {
            if let MetaData::Go(go) = data {
                for (k, v) in go.func_list.iter() {
                    maps.insert(k.clone(), v.clone());
                }
            }
        }
        maps
    }
    pub fn go_new_func_maps(&self) -> HashMap<String, XMethod> {
        let mut maps = HashMap::new();
        if let Some(data) = &self.data {
            if let MetaData::Go(go) = data {
                for (k, v) in go.new_func_list.iter() {
                    maps.insert(k.clone(), v.clone());
                }
            }
        }
        maps
    }
}

pub enum ProcessType {
    ProcessTypeMiman,
}

#[derive(Debug, Clone)]
pub enum GenerateType {
    GenerateTypeMiman,
    GenerateTypeSgz,
}
impl Default for GenerateType {
    fn default() -> Self {
        GenerateType::GenerateTypeMiman
    }
}
#[derive(Debug, Clone)]
pub struct GenerateData {
    pub path: String,
    pub gen_type: GenerateType,
    pub out_type: OutputType,
    pub content: String,
    pub seq: u32,
}
impl Default for GenerateData {
    fn default() -> Self {
        Self {
            path: Default::default(),
            gen_type: Default::default(),
            out_type: Default::default(),
            content: Default::default(),
            seq: 9999,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum OutputType {
    OutputTypeGo,
    OutputTypeMd,
}

impl Default for OutputType {
    fn default() -> Self {
        OutputType::OutputTypeGo
    }
}
