use crate::core::register_parser;

mod gm_parser;

pub fn register() {
    register_parser("gm", Box::new(gm_parser::GMParser {}))
}
