use crate::core::register_outputer;

pub mod go_outputer;
pub mod md_outputer;
pub fn register() {
    register_outputer("go", Box::new(go_outputer::GoOutputer {}));
    register_outputer("md", Box::new(md_outputer::MdOutputer {}));
}
