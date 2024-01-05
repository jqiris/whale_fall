use crate::core::{
    meta::{MetaNode, ParserType},
    traits::IParser,
};
use anyhow::Result;
use std::{fmt, path::Path};

//通用模型解析器
pub struct GMParser {}

impl IParser for GMParser {
    fn parser_type(&self) -> ParserType {
        ParserType::ParserTypeGM
    }
    fn parse(&self, root: &Path) -> Result<MetaNode> {
        todo!()
    }
}

impl fmt::Display for GMParser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gm")
    }
}
