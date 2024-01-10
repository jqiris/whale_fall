use crate::{
    common::option_to_result,
    core::{meta::*, traits::IOutputer},
};
use anyhow::{anyhow, Result};
use core::fmt;
use std::{fs, path::Path, process::Command};
pub struct GoOutputer {}

impl fmt::Display for GoOutputer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "go")
    }
}

impl IOutputer for GoOutputer {
    fn output_type(&self) -> OutputType {
        OutputType::OutputTypeGo
    }

    fn output(&self, data: Vec<GenerateData>) -> Result<()> {
        for item in data {
            if item.out_type != self.output_type() {
                continue;
            }
            self.produce(&item)?;
        }
        Ok(())
    }
}

impl GoOutputer {
    fn produce(&self, data: &GenerateData) -> Result<()> {
        //创建目录
        let file = Path::new(&data.path);
        let parent = option_to_result(file.parent(), anyhow!("invalid path"))?;
        fs::create_dir_all(parent)?;
        fs::write(file, &data.content)?;
        let output = Command::new("gofmt").arg(&file).output()?;
        if !output.status.success() {
            return Err(anyhow!(
                "gofmt error: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(())
    }
}
