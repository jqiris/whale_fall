pub mod meta;
#[cfg(test)]
mod tests;
pub mod traits;
use anyhow::Result;
use lazy_static::lazy_static;
use std::{collections::HashMap, path::Path, sync::Mutex};
use traits::*;

use self::meta::*;

lazy_static! {
    static ref PARSER: Mutex<HashMap<String, Box<dyn IParser + Send + Sync>>> = {
        let maps = HashMap::new();
        Mutex::new(maps)
    };
    static ref PROCESSER: Mutex<HashMap<String, Box<dyn IProcesser + Send + Sync>>> = {
        let maps = HashMap::new();
        Mutex::new(maps)
    };
    static ref GENERATOR: Mutex<HashMap<String, Box<dyn IGenerator + Send + Sync>>> = {
        let maps = HashMap::new();
        Mutex::new(maps)
    };
    static ref OUTPUTER: Mutex<HashMap<String, Box<dyn IOutputer + Send + Sync>>> = {
        let maps = HashMap::new();
        Mutex::new(maps)
    };
}

pub fn register_parser(name: &str, parser: Box<dyn IParser + Send + Sync>) {
    let mut maps = PARSER.lock().unwrap();
    maps.insert(name.to_string(), parser);
}
pub fn register_processer(name: &str, processer: Box<dyn IProcesser + Send + Sync>) {
    let mut maps = PROCESSER.lock().unwrap();
    maps.insert(name.to_string(), processer);
}
pub fn register_generator(name: &str, generator: Box<dyn IGenerator + Send + Sync>) {
    let mut maps = GENERATOR.lock().unwrap();
    maps.insert(name.to_string(), generator);
}
pub fn register_outputer(name: &str, outputer: Box<dyn IOutputer + Send + Sync>) {
    let mut maps = OUTPUTER.lock().unwrap();
    maps.insert(name.to_string(), outputer);
}

pub fn parse(name: &str, root: &str) -> Result<MetaNode> {
    match PARSER.lock().unwrap().get(name) {
        Some(parser) => parser.parse(Path::new(root)),
        None => Err(anyhow::anyhow!("parser {} not found", name)),
    }
}

pub fn process(name: &str, data: MetaNode) -> Result<ProcessData> {
    match PROCESSER.lock().unwrap().get(name) {
        Some(processer) => processer.process(data),
        None => Err(anyhow::anyhow!("processer {} not found", name)),
    }
}

pub fn generate(name: &str, root: &str, pkg: &str, data: ProcessData) -> Result<Vec<GenerateData>> {
    match GENERATOR.lock().unwrap().get(name) {
        Some(generator) => generator.generate(root, pkg, data),
        None => Err(anyhow::anyhow!("generator {} not found", name)),
    }
}

pub fn output(name: &str, data: Vec<GenerateData>) -> Result<()> {
    match OUTPUTER.lock().unwrap().get(name) {
        Some(outputer) => outputer.output(data),
        None => Err(anyhow::anyhow!("outputer {} not found", name)),
    }
}
