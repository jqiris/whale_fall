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
        let app_list = meta.find_list_by_name("cmd");
        let micro_list = meta.find_list_by_name("micro");
        result.lists.insert("app_list".to_string(), app_list);
        result.lists.insert("micro_list".to_string(), micro_list);
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
