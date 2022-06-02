use crate::path::root_path;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryConfig {
    pub binaries: HashMap<String, BinaryInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PackageManagerTool {
    Npm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryInfo {
    pub tool: PackageManagerTool,
    pub installed: Vec<BinaryInstalled>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BinaryInstalled {
    pub version: String,
    pub arch: String,
    pub bin: String,
}

pub fn binary_config() -> Result<BinaryConfig> {
    let config_path = root_path().join("binary.yml");
    if fs::metadata(&config_path).is_ok() {
        let config_str = fs::read_to_string(&config_path)?;
        let config: BinaryConfig = serde_yaml::from_str(&config_str)?;
        Ok(config)
    } else {
        Ok(BinaryConfig {
            binaries: Default::default(),
        })
    }
}

pub fn write_binary_config(config: &BinaryConfig) -> Result<()> {
    let config_path = root_path().join("binary.yml");
    fs::write(&config_path, serde_yaml::to_string(&config)?)?;
    Ok(())
}
