use super::meta::*;
use anyhow::Result;
use core::fmt;
use std::path::Path;

pub trait IParser: fmt::Display {
    fn parser_type(&self) -> ParserType;
    fn parse(&self, path: &Path) -> Result<MetaNode>;
}

pub trait IProcesser: fmt::Display {
    fn process_type(&self) -> ProcessType;
    fn process(&self, meta: &mut MetaNode) -> Result<()>;
}

pub trait IGenerator: fmt::Display {
    fn generate_type(&self) -> GenerateType;
    fn generate(&self, root: &str, pkg: &str, meta: &MetaNode) -> Result<Vec<GenerateData>>;
}

pub trait IOutputer: fmt::Display {
    fn output_type(&self) -> OutputType;
    fn output(&self, data: Vec<GenerateData>) -> Result<()>;
}
