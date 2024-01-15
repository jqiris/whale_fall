use crate::core::register_generator;

mod miman_generator;
#[cfg(test)]
mod tests;

pub fn register() {
    register_generator("miman", Box::new(miman_generator::MimanGenerator {}))
}
