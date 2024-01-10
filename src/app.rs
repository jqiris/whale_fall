use std::fs::read_to_string;

use anyhow::Result;
use clap::{command, Args, Parser, Subcommand};
use serde::Deserialize;

use crate::core::{generate, output, parse, process};

#[derive(Debug, Deserialize)]
pub struct Basic {
    pub package: String, //包名
    pub root: String,    //根目录
}
#[derive(Debug, Deserialize)]
pub struct Composer {
    pub parsers: Vec<String>,    //解析器链表
    pub processers: Vec<String>, //处理器链表
    pub generators: Vec<String>, //生成器链表
    pub outputers: Vec<String>,  //输出器链表
}
#[derive(Debug, Deserialize)]
pub struct Config {
    pub basic: Basic,
    pub composer: Vec<Composer>,
}
#[derive(Parser)]
#[command(name = "whale_fall")]
#[command(author = "jqiris <1920624985@qq.com>")]
#[command(version = "1.0")]
#[command(about = "When a whale falls, all things live", long_about = None)]
#[command(propagate_version = true)]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

impl App {
    pub fn generate(&self, args: &GenerateParam) -> Result<()> {
        let mut cfg_file = "whale.toml";
        if let Some(data) = &args.config {
            cfg_file = data;
        }
        let cfg = toml::from_str::<Config>(&read_to_string(cfg_file)?)?;
        println!("cfg:{:?}", cfg);
        for composer in cfg.composer {
            let mut meta_list = Vec::new();
            for parser in composer.parsers {
                meta_list.push(parse(&parser, &cfg.basic.root)?);
            }
            let mut process_list = Vec::new();
            for processer in composer.processers {
                for meta in &meta_list {
                    process_list.push(process(&processer, meta.clone())?);
                }
            }
            let mut generate_list = Vec::new();
            for generator in composer.generators {
                for process in &process_list {
                    let mut gen_list = generate(&generator, &cfg.basic.package, process.clone())?;
                    generate_list.append(&mut gen_list);
                }
            }
            for outputer in composer.outputers {
                output(&outputer, generate_list.clone())?;
            }
        }
        Ok(())
    }
}
#[derive(Debug, Args)]
pub struct GenerateParam {
    #[arg(short, long, help = "The config file of the project")]
    pub config: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// generate
    Generate(GenerateParam),
}
