use crate::core::register_processer;

mod miman_processer;

pub fn register() {
    register_processer("miman", Box::new(miman_processer::MimanProcesser {}))
}
