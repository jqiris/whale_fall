use core::fmt;
use std::collections::HashMap;

use crate::core::{
    meta::{MetaNode, ProcessData, ProcessType},
    traits::IProcesser,
};
use anyhow::Result;

pub struct MimanProcesser {}

impl IProcesser for MimanProcesser {
    fn process(&self, meta: MetaNode) -> Result<ProcessData> {
        let mut result = ProcessData {
            lists: HashMap::new(),
            maps: HashMap::new(),
        };
        let business = meta.find_by_name("business");
        let micro = meta.find_by_name("micro");
        if let Some(data) = business {
            result.maps.insert("business".to_string(), data);
        }
        if let Some(data) = micro {
            result.maps.insert("micro".to_string(), data);
        }

        let excludes = vec!["cmd", "config", "entity", "jobs", "scripts"];
        let gi_list = meta.find_gi_list(&excludes);
        result.lists.insert("gi".to_string(), gi_list);
        Ok(result)
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
