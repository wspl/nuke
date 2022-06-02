use anyhow::{anyhow, Result};
use clap::ArgMatches;
use colored::*;
use nuke_shared::{
    binary::{write_binary_config, BinaryConfig, BinaryInfo, BinaryInstalled, PackageManagerTool},
    path::root_path,
};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
};

#[cfg(debug_assertions)]
static LAUNCHER_BIN: &[u8] = include_bytes!("../../target/debug/nuke-launcher.exe");
#[cfg(not(debug_assertions))]
static LAUNCHER_BIN: &[u8] = include_bytes!("../../target/release/nuke-launcher.exe");

pub fn versions_iter() -> Result<impl Iterator<Item = Result<(String, String, PathBuf)>>> {
    let versions_dir = root_path().join("versions");

    let iter = fs::read_dir(&versions_dir)?.map(|dir| {
        let dir = dir?;
        let file_name = dir.file_name();
        let mut splitted = file_name.to_str().unwrap().split("-");
        let version = splitted.next().unwrap();
        let arch = splitted.next().unwrap();

        Ok((version.to_string(), arch.to_string(), dir.path()))
    });

    Ok(iter)
}

pub fn scan_bin(matches: Option<&ArgMatches>) -> Result<()> {
    let mut version_map = HashMap::<String, BinaryInfo>::new();
    let bin_script_regex = Regex::new(r####"node_modules\\[^"]+"####)?;

    for item in versions_iter()? {
        let (version, arch, node_path) = item?;

        for dir in fs::read_dir(node_path)? {
            let dir = dir?;
            let script_path = dir.path();
            if let Some(ext) = script_path.extension() {
                let ext = ext.to_str().unwrap();
                if ext == "cmd" {
                    let file_name = dir.file_name();
                    let file_name_str = file_name.to_str().unwrap();
                    let bin_name = &file_name_str[0..file_name_str.len() - 4];

                    if !["npm", "npx"].contains(&bin_name) {
                        let script = fs::read_to_string(&script_path)?;
                        let bin_matched = bin_script_regex
                            .find(script.as_str())
                            .ok_or(anyhow!("Cannot find bin script from cmd file"))?;
                        let bin = bin_matched.as_str();

                        if let Some(info) = version_map.get_mut(bin_name) {
                            info.installed.push(BinaryInstalled {
                                version: version.to_string(),
                                arch: arch.to_string(),
                                bin: bin.to_string(),
                            })
                        } else {
                            let info = BinaryInfo {
                                tool: PackageManagerTool::Npm,
                                installed: vec![BinaryInstalled {
                                    version: version.to_string(),
                                    arch: arch.to_string(),
                                    bin: bin.to_string(),
                                }],
                            };
                            version_map.insert(bin_name.to_string(), info);
                        }
                    }
                }
            }
        }
    }

    // Copy launcher to bin folder
    let primary_command = vec!["node", "npm", "npx", "corepack"];
    let bin_dir = root_path().join("bin");
    for command in &primary_command {
        let _ = fs::write(bin_dir.join(format!("{}.exe", command)), LAUNCHER_BIN);
    }
    let exists_command = fs::read_dir(&bin_dir)?
        .filter_map(|p| {
            p.ok()
                .and_then(|v| v.file_name().to_str().map(|s| s.to_string()))
        })
        .filter(|c| c.ends_with(".exe"))
        .map(|c| (&c[..c.len() - 4]).to_string())
        .filter(|c| !primary_command.contains(&c.as_str()));
    for node_command in exists_command {
        let _ = fs::remove_file(bin_dir.join(format!("{}.exe", node_command)));
    }
    for node_command in version_map.keys() {
        let _ = fs::write(bin_dir.join(format!("{}.exe", node_command)), LAUNCHER_BIN);
    }

    // Write to binaries config
    let config = BinaryConfig {
        binaries: version_map,
    };
    write_binary_config(&config)?;

    if !matches.map_or(false, |m| m.is_present("silent")) {
        println!(
            "{} The indexing of binaries has been completed.",
            "[Nuke]".green()
        );
    }

    Ok(())
}
