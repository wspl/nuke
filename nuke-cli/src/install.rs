use crate::{download::Downloader, scan_bin::scan_bin};
use anyhow::{anyhow, bail, Result};
use clap::ArgMatches;
use colored::*;

use nuke_shared::{
    node::{node_config, write_node_config, NodeConfigInstalled, NodeConfigVersion},
    path::root_path,
};
use regex::Regex;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::{
    fs::{self, File},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeVersion {
    pub version: String,
    pub date: String,
    pub files: Vec<String>,
    pub npm: Option<String>,
    pub v8: Option<String>,
    pub uv: Option<String>,
    pub zlib: Option<String>,
    pub openssl: Option<String>,
    pub modules: Option<String>,
    pub lts: NodeLTS,
    pub security: bool,
}

impl NodeVersion {
    pub fn version(&self) -> String {
        (&self.version[1..]).to_string()
    }
}

impl NodeVersion {
    fn semver(&self) -> Version {
        Version::parse(&self.version[1..]).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum NodeLTS {
    Bool(bool),
    Name(String),
}

impl NodeLTS {
    pub fn is_lts(&self) -> bool {
        match self {
            NodeLTS::Bool(v) => *v,
            NodeLTS::Name(_) => true,
        }
    }
}

pub fn list_versions() -> Result<Vec<NodeVersion>> {
    let res =
        reqwest::blocking::get("https://nodejs.org/dist/index.json")?.json::<Vec<NodeVersion>>()?;
    Ok(res)
}

pub fn get_filtered_latest<P>(list: &Vec<NodeVersion>, condition: P) -> Option<NodeVersion>
where
    P: FnMut(&NodeVersion) -> bool,
{
    let mut v = list
        .clone()
        .into_iter()
        .filter(condition)
        .collect::<Vec<NodeVersion>>();
    v.sort_by(|a, b| a.semver().cmp(&b.semver()));
    v.pop()
}

pub fn install_node(matches: &ArgMatches) -> Result<()> {
    // Supported Node.js version patterns：
    // 16 / 16.1 / 16.1.0 / lts / latest

    let input_version = matches.value_of("VERSION").unwrap();
    let arch = matches.value_of("arch").unwrap_or("x64");

    let versions = list_versions()?
        .into_iter()
        .filter(|v| v.files.contains(&format!("win-{}-7z", arch)))
        .collect::<Vec<NodeVersion>>();

    let matched_version = {
        if input_version == "lts" {
            get_filtered_latest(&versions, |v| v.lts.is_lts())
        } else if input_version == "latest" {
            get_filtered_latest(&versions, |_| true)
        } else if Regex::new(r"^\d+$")?.is_match(input_version) {
            let version_req = VersionReq::parse(format!("{}.*.*", input_version).as_str())?;
            get_filtered_latest(&versions, |v| version_req.matches(&v.semver()))
        } else if Regex::new(r"^\d+\.\d+$")?.is_match(input_version) {
            let version_req = VersionReq::parse(format!("{}.*", input_version).as_str())?;
            get_filtered_latest(&versions, |v| version_req.matches(&v.semver()))
        } else if Regex::new(r"^\d+\.\d+\.\d+$")?.is_match(input_version) {
            get_filtered_latest(&versions, |v| input_version == v.version())
        } else {
            let version_req = VersionReq::parse(input_version);
            match version_req {
                Ok(version_req) => {
                    get_filtered_latest(&versions, |v| version_req.matches(&v.semver()))
                }
                Err(_) => {
                    bail!("invalid version pattern: {}", input_version)
                }
            }
        }
    };

    let matched_version = matched_version.ok_or(anyhow!(
        "unable to find a version that matches the specified version pattern"
    ))?;

    println!(
        "{} Matched version: {}",
        "[Nuke]".green(),
        format!("{} ({})", matched_version.version(), arch).yellow()
    );
    let url = format!(
        "https://nodejs.org/dist/{}/node-{}-win-{}.7z",
        matched_version.version, matched_version.version, arch
    );

    let hash_hex = {
        let mut hasher = Sha1::new();
        hasher.update(&url);
        let hash = hasher.finalize();
        format!("{:02x}", hash)
    };

    let target = root_path()
        .join("temp")
        .join(format!("{}.nukedownload", hash_hex));
    let dl = Downloader::new(&url, &target);
    dl.start_download()?;

    println!("{} Installing...", "[Nuke]".green());
    let decompressed_dest = root_path().join("temp").join(hash_hex);

    {
        let mut file = File::open(&target)?;
        compress_tools::uncompress_archive(
            &mut file,
            &decompressed_dest,
            compress_tools::Ownership::Preserve,
        )?;
    }

    let inner_dir = fs::read_dir(&decompressed_dest)?.next().unwrap()?.path();

    let versions_dir = root_path().join("versions");
    let version_name = format!("{}-{}", matched_version.version(), arch);
    let version_dir = versions_dir.join(version_name);
    let _ = fs::remove_dir_all(&version_dir);
    fs::rename(&inner_dir, &version_dir)?;

    let _ = fs::remove_dir_all(&decompressed_dest);
    let _ = fs::remove_file(&target);

    // remove default buggy npm/npx command file
    let _ = fs::remove_file(version_dir.join("npm.cmd"));
    let _ = fs::remove_file(version_dir.join("npm"));
    let _ = fs::remove_file(version_dir.join("npx.cmd"));
    let _ = fs::remove_file(version_dir.join("npx"));

    let mut node_config = node_config()?;
    let new_installed = NodeConfigInstalled {
        version: matched_version.version(),
        arch: arch.to_string(),
        lts: matched_version.lts.is_lts(),
    };
    if let Some(exist_index) = node_config
        .installed
        .iter()
        .position(|t| t.eq(&new_installed))
    {
        node_config.installed.remove(exist_index);
    }
    node_config.installed.push(new_installed);

    if node_config.installed.len() == 1 {
        node_config.default.replace(NodeConfigVersion {
            version: matched_version.version(),
            arch: arch.to_string(),
        });
    }

    write_node_config(&node_config)?;

    println!(
        "{} {} has been successfully installed.",
        "[Nuke]".green(),
        format!("{} ({})", matched_version.version(), arch).yellow()
    );

    if node_config.installed.len() == 1 {
        println!(
            "{} This version has been set as the default version.",
            "[Nuke]".green()
        );
    } else {
        println!(
            "{} You can set this version as the default using the command: {}",
            "[Nuke]".green(),
            format!("nuke default {} --arch {}", matched_version.version(), arch).bright_black()
        );
    }

    scan_bin(None)?;

    Ok(())
}
