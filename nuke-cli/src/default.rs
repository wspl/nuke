use anyhow::{anyhow, Result};
use clap::ArgMatches;
use colored::*;
use nuke_shared::node::{node_config, write_node_config};

pub fn set_default_node(matches: &ArgMatches) -> Result<()> {
    let mut node_config = node_config()?;

    let arch = matches.value_of("arch").unwrap_or("x64");
    let input_version = matches.value_of("VERSION").unwrap();

    let matched_version = node_config.match_version(input_version, Some(arch))?;

    let matched_version = matched_version.ok_or(anyhow!(
        "Unable to find a installed version that matches the specified version pattern"
    ))?;

    node_config.default.replace(matched_version.clone().into());

    write_node_config(&node_config)?;

    println!(
        "{} {} has been set as the default version.",
        "[Nuke]".green(),
        format!("{} ({})", matched_version.version, matched_version.version).yellow()
    );

    Ok(())
}
