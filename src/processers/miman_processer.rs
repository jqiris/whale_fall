use core::fmt;
use std::collections::HashMap;

use crate::core::{
    meta::{ExtraData, MetaNode, ProcessType},
    traits::IProcesser,
};
use anyhow::Result;

pub struct MimanProcesser {}

impl IProcesser for MimanProcesser {
    fn process(&self, meta: &mut MetaNode) -> Result<()> {
        let mut extra_data = HashMap::new();
        let excludes = vec!["cmd", "config", "entity", "jobs", "scripts"];
        let gi_list = meta.find_gi_list(&excludes);
        if gi_list.len() > 0 {
            extra_data.insert("gi".to_string(), ExtraData::MetaList(gi_list));
        }
        meta.extra_data = Some(extra_data);
        Ok(())
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
