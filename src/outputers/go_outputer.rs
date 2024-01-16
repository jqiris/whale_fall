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
        self.check_go_imports()?;
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
    fn check_go_imports(&self) -> Result<()> {
        if let Err(_) = Command::new("goimports").arg("--version").output() {
            let install_output = Command::new("go")
                .arg("install")
                .arg("golang.org/x/tools/cmd/goimports@latest")
                .output()?;
            if install_output.status.success() {
                println!("goimports command installed successfully");
            } else {
                return Err(anyhow!("Failed to install goimports command"));
            }
        }
        Ok(())
    }
    fn produce(&self, data: &GenerateData) -> Result<()> {
        //创建目录
        let file = Path::new(&data.path);
        let parent = option_to_result(file.parent(), anyhow!("invalid path"))?;
        fs::create_dir_all(parent)?;
        fs::write(file, &data.content)?;
        let output = Command::new("goimports").arg("-w").arg(&file).output()?;
        if !output.status.success() {
            return Err(anyhow!(
                "goimports error: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(())
    }
}
