mod common;
mod core;
mod parsers;
mod processers;
fn main() {
    register();
}

fn register() {
    parsers::register();
    processers::register();
}
