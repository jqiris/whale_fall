use crate::{
    common::file::{path_name, path_str},
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
                let mut metaNode = MetaNode {
                    name: path_name(pwd),
                    path: path_str(pwd),
                    is_dir: metadata.is_dir(),
                    child: Vec::new(),
                    data: None,
                };
                if metadata.is_file() {
                    let ext = pwd.extension();
                    if ext == Some(OsStr::new("go")) {
                        let ast_file = parse_file(pwd)?;
                    }
                } else if metadata.is_dir() {
                }
                Ok(metaNode)
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
