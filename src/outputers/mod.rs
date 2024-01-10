use crate::core::register_outputer;

pub mod go_outputer;
pub fn register() {
    register_outputer("go", Box::new(go_outputer::GoOutputer {}))
}
