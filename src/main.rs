mod common;
mod core;
mod generators;
mod parsers;
mod processers;
mod tpls;
fn main() {
    register();
}

fn register() {
    parsers::register();
    processers::register();
    generators::register();
}
