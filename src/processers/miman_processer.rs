use core::fmt;

use crate::core::{
    meta::{MetaNode, ProcessData, ProcessType},
    traits::IProcesser,
};
use anyhow::Result;

pub struct MimanProcesser {}

impl IProcesser for MimanProcesser {
    fn process(&self, meta: MetaNode) -> Result<ProcessData> {
        todo!()
    }

    fn process_type(&self) -> ProcessType {
        ProcessType::ProcessTypeMiman
    }
}

impl fmt::Display for MimanProcesser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "miman")
    }
}
