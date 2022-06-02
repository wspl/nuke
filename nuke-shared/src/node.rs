use std::fs;

use crate::path::root_path;
use anyhow::{bail, Result};
use regex::Regex;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    pub default: Option<NodeConfigVersion>,
    pub installed: Vec<NodeConfigInstalled>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeConfigVersion {
    pub version: String,
    pub arch: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct NodeConfigInstalled {
    pub version: String,
    pub arch: String,
    pub lts: bool,
}

impl NodeConfigInstalled {
    pub fn eq_version(&self, v: &NodeConfigVersion) -> bool {
        self.version == v.version && self.arch == v.arch
    }
}

impl Into<NodeConfigVersion> for NodeConfigInstalled {
    fn into(self) -> NodeConfigVersion {
        NodeConfigVersion {
            version: self.version,
            arch: self.arch,
        }
    }
}

pub fn node_config() -> Result<NodeConfig> {
    let node_config_path = root_path().join("node.yml");
    if fs::metadata(&node_config_path).is_ok() {
        let node_config_str = fs::read_to_string(&node_config_path)?;
        let node_config: NodeConfig = serde_yaml::from_str(&node_config_str)?;
        Ok(node_config)
    } else {
        Ok(NodeConfig {
            default: None,
            installed: vec![],
        })
    }
}

pub fn write_node_config(config: &NodeConfig) -> Result<()> {
    let node_config_path = root_path().join("node.yml");
    fs::write(&node_config_path, serde_yaml::to_string(config)?)?;
    Ok(())
}

pub fn get_filtered_latest<P>(
    list: &Vec<NodeConfigInstalled>,
    mut condition: P,
) -> Option<NodeConfigInstalled>
where
    P: FnMut(&(Version, String)) -> bool,
{
    let mut v = list
        .iter()
        .filter(|item| {
            let version = Version::parse(&item.version).unwrap();
            condition(&(version, item.arch.clone()))
        })
        .collect::<Vec<&NodeConfigInstalled>>();
    v.sort_by(|a, b| a.version.cmp(&b.version));
    v.pop().map(|t| t.clone())
}

impl NodeConfig {
    pub fn match_version(
        &self,
        input_version: &str,
        arch: Option<&str>,
    ) -> Result<Option<NodeConfigInstalled>> {
        let matched_version = if Regex::new(r"^\d+$")?.is_match(input_version) {
            let version_req = VersionReq::parse(format!("{}.*.*", input_version).as_str())?;
            get_filtered_latest(&self.installed, |(v, a)| {
                version_req.matches(&v) && arch.map_or(true, |arch| arch == a)
            })
        } else if Regex::new(r"^\d+\.\d+$")?.is_match(input_version) {
            let version_req = VersionReq::parse(format!("{}.*", input_version).as_str())?;
            get_filtered_latest(&self.installed, |(v, a)| {
                version_req.matches(&v) && arch.map_or(true, |arch| arch == a)
            })
        } else {
            let version_req = VersionReq::parse(input_version);
            match version_req {
                Ok(version_req) => get_filtered_latest(&self.installed, |(v, a)| {
                    version_req.matches(&v) && arch.map_or(true, |arch| arch == a)
                }),
                Err(_) => {
                    bail!("Invalid version pattern: {}", input_version)
                }
            }
        };
        Ok(matched_version)
    }
}
