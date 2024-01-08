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
    fn process(&self, data: MetaNode) -> Result<ProcessData>;
}

pub trait IGenerator: fmt::Display {
    fn generate_type(&self) -> GenerateType;
    fn generate(&self, data: ProcessData) -> Result<Vec<GenerateData>>;
}

pub trait IOutputer: fmt::Display {
    fn output_type(&self) -> OutputType;
    fn output(&self, data: Vec<GenerateData>) -> Result<()>;
}
