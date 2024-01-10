use app::*;
use clap::Parser;
mod app;
mod common;
mod core;
mod generators;
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
}

fn generate() {
    let app = App::parse();
    let res = match &app.command {
        Commands::Generate(args) => app.generate(args),
    };
    if let Err(err) = res {
        println!("exec error: {}", err);
    }
}
