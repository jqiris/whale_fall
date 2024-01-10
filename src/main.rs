use app::*;
use clap::Parser;
mod app;
mod common;
mod core;
mod generators;
mod outputers;
mod parsers;
mod processers;
mod tpls;
fn main() {
    register();
    generate();
}

fn register() {
    parsers::register();
    processers::register();
    generators::register();
    outputers::register();
}

fn generate() {
    let app = App::parse();
    let res = match &app.command {
        Commands::Generate(args) => app.generate(args),
    };
    if let Err(err) = res {
        println!("generate error: {}", err);
    } else {
        println!("generate success");
    }
}
