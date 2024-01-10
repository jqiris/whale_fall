use crate::{
    common::{
        file::{path_name, path_str},
        go::MetaGo,
    },
    core::{meta::*, traits::IParser},
};
use anyhow::{anyhow, Result};
use gosyn::parse_file;
use std::{ffi::OsStr, fmt, fs, path::Path};

//通用模型解析器
pub struct GMParser {}

impl IParser for GMParser {
    fn parser_type(&self) -> ParserType {
        ParserType::ParserTypeGM
    }
    fn parse(&self, pwd: &Path) -> Result<MetaNode> {
        match fs::metadata(pwd) {
            Ok(metadata) => {
                if !metadata.is_dir() && !metadata.is_file() {
                    return Err(anyhow!("{} is not file or dir", pwd.display()));
                }
                let mut meta_node = MetaNode {
                    name: path_name(pwd),
                    path: path_str(pwd),
                    is_dir: metadata.is_dir(),
                    childs: Vec::new(),
                    data: None,
                };
                if metadata.is_file() {
                    let ext = pwd.extension();
                    if ext == Some(OsStr::new("go")) {
                        let ast_file = parse_file(pwd)?;
                        let mut meta_go = MetaGo::from(ast_file);
                        meta_go.load_binds();
                        meta_node.data = Some(MetaData::Go(meta_go));
                    } else if ext == Some(OsStr::new("md")) {
                        let content = fs::read_to_string(pwd)?;
                        meta_node.data = Some(MetaData::Md(content));
                    }
                } else if metadata.is_dir() {
                    for entry in fs::read_dir(pwd)? {
                        let data = entry?;
                        let (file_type, file_path) = (data.file_type()?, data.path());
                        if file_type.is_file() || file_type.is_dir() {
                            let child = self.parse(&file_path)?;
                            meta_node.childs.push(child);
                        }
                    }
                    let mut meta_go = MetaGo::default();
                    for child in meta_node.childs.iter() {
                        if let Some(data) = &child.data {
                            if let MetaData::Go(go) = data {
                                meta_go.merge(&go);
                            }
                        }
                    }
                    meta_go.load_binds();
                    meta_node.data = Some(MetaData::Go(meta_go));
                }
                Ok(meta_node)
            }
            Err(err) => Err(anyhow!(err)),
        }
    }
}

impl fmt::Display for GMParser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gm")
    }
}
