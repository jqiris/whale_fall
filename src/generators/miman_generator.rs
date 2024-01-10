use core::fmt;

use crate::core::{meta::*, traits::IGenerator};

pub struct MimanGenerator {}

impl fmt::Display for MimanGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "miman")
    }
}

impl IGenerator for MimanGenerator {
    fn generate_type(&self) -> GenerateType {
        GenerateType::GenerateTypeMiman
    }

    fn generate(&self, pkg: &str, data: ProcessData) -> anyhow::Result<Vec<GenerateData>> {
        todo!()
    }
}
