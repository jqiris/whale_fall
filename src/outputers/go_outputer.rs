use crate::core::{meta::*, traits::IOutputer};
use anyhow::Result;
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
        fs::create_dir_all(file.parent().unwrap())?;
        fs::write(file, &data.content)?;
        Command::new("gofmt").arg(&file).spawn()?;
        Ok(())
    }
}
