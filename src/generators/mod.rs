use crate::core::register_generator;

mod miman_generator;
mod sgz_generator;
#[cfg(test)]
mod tests;

pub fn register() {
    register_generator("miman", Box::new(miman_generator::MimanGenerator {}));
    register_generator("sgz", Box::new(sgz_generator::SgzGenerator {}));
}
