use anyhow::Result;
use colored::*;
use nuke_shared::node::node_config;

pub fn list_installed_node() -> Result<()> {
    println!(
        "{} You have installed the following versions of Node.js:",
        "[Nuke]".green()
    );
    let node_config = node_config()?;
    for installed in node_config.installed {
        let is_default = if let Some(default) = &node_config.default {
            installed.eq_version(default)
        } else {
            false
        };
        println!(
            "       {} {} ({}) {}",
            if is_default {
                format!("- [{}]", "*".yellow()).bright_black()
            } else {
                "- [ ]".bright_black()
            },
            installed.version,
            installed.arch,
            if is_default {
                "- default".yellow()
            } else {
                "".into()
            }
        );
    }
    Ok(())
}
