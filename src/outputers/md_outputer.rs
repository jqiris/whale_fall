use crate::{
    common::option_to_result,
    core::{meta::*, traits::IOutputer},
};
use anyhow::{anyhow, Result};
use core::fmt;
use std::{fs, path::Path};

pub struct MdOutputer {}

impl fmt::Display for MdOutputer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "md")
    }
}

impl IOutputer for MdOutputer {
    fn output_type(&self) -> OutputType {
        OutputType::OutputTypeMd
    }

    fn output(&self, data: Vec<GenerateData>) -> anyhow::Result<()> {
        for item in data {
            if item.out_type != self.output_type() {
                continue;
            }
            self.produce(&item)?;
        }
        Ok(())
    }
}

impl MdOutputer {
    fn produce(&self, data: &GenerateData) -> Result<()> {
        //创建目录
        let file = Path::new(&data.path);
        let parent = option_to_result(file.parent(), anyhow!("invalid path"))?;
        fs::create_dir_all(parent)?;
        fs::write(file, &data.content)?;
        Ok(())
    }
}
