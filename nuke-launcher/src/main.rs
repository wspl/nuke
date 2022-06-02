use anyhow::{anyhow, bail, Result};
use colored::*;
use nuke_shared::{
    binary::binary_config,
    node::{node_config},
    path::root_path,
};
use std::{
    collections::HashMap,
    env,
    process::{Command},
};

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => println!("{}", e.to_string()),
    }
}

fn run_nuke(args: Vec<&str>) -> Result<()> {
    let nuke_path = root_path().join("bin").join("nuke.exe");
    Command::new(nuke_path).args(args).spawn()?.wait()?;
    Ok(())
}

fn run() -> Result<()> {
    let node_config = node_config()?;
    let binary_config = binary_config()?;

    let node_version = if let Some(version) = env::var("NUKE_NODE_VERSION").ok() {
        let arch = env::var("NUKE_NODE_ARCH").ok();
        node_config
            .match_version(version.as_str(), arch.as_deref())?
            .map(|t| t.into())
            .ok_or(anyhow!("Node version is not found"))?
    } else {
        node_config
            .default
            .ok_or(anyhow!("Default node version is unset"))?
    };

    let node_root = root_path()
        .join("versions")
        .join(format!("{}-{}", node_version.version, node_version.arch));
    let node_path = node_root.join("node.exe");

    let current_exe = env::current_exe().unwrap();
    let file_name = current_exe.file_name().unwrap().to_str().unwrap();
    let binary_name = &file_name[..file_name.len() - 4];

    let npm_bin_dir = node_root.join("node_modules").join("npm").join("bin");
    let npm_cli_path = npm_bin_dir.join("npm-cli.js");
    let npx_cli_path = npm_bin_dir.join("npx-cli.js");

    let mut envs = HashMap::new();
    envs.insert("NODE_EXE", node_path.to_str().unwrap().to_string());
    envs.insert("NPM_CLI_JS", npm_cli_path.to_str().unwrap().to_string());
    envs.insert("NPX_CLI_JS", npx_cli_path.to_str().unwrap().to_string());

    match binary_name {
        "node" => {
            let args = env::args().skip(1);
            Command::new(&node_path).args(args).spawn()?.wait()?;
        }
        "npm" => {
            let mut is_global = false;
            for arg in env::args() {
                if ["-g", "--global"].contains(&arg.as_str()) {
                    is_global = true;
                }
            }
            Command::new(node_path)
                .arg(npm_cli_path)
                .args(env::args().skip(1))
                .arg("--scripts-prepend-node-path=true")
                .envs(envs)
                .spawn()?
                .wait()?;
            if is_global {
                run_nuke(vec!["scan-bin", "--silent"])?;
            }
        }
        "npx" => {
            Command::new(node_path)
                .arg(npx_cli_path)
                .args(env::args().skip(1))
                .envs(envs)
                .spawn()?
                .wait()?;
        }
        _ => {
            if let Some(info) = binary_config.binaries.get(binary_name) {
                let installed = info
                    .installed
                    .iter()
                    .find(|t| t.version == node_version.version && t.arch == node_version.arch);
                if let Some(installed) = installed {
                    let bin_path = node_root.join(&installed.bin);
                    Command::new(node_path)
                        .arg(bin_path)
                        .args(env::args().skip(1))
                        .spawn()?
                        .wait()?;
                } else {
                    bail!(
                        "{}: \"{}\" is not yet installed in the currently version of Node.js, you can install it via npm, or switch to these Node.js versions: {}.",
                        binary_name.bright_blue(),
                        binary_name,
                        info.installed.iter().map(|t| format!("{} ({})", t.version, t.arch).yellow().to_string()).collect::<Vec<String>>().join(", ")
                    )
                }
            } else {
                bail!(
                    "No version of Node.js was found for the aaa command, try running {}.",
                    "nuke scan-bin".bright_black()
                )
            }
        }
    };

    Ok(())
}
