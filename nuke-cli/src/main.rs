extern crate lazy_static;

mod default;
mod download;
mod install;
mod list;
mod scan_bin;
mod setup;

use std::{env::current_exe};

use clap::{Command, AppSettings, Arg};
use colored::*;
use dialoguer::Confirm;
use nuke_shared::path::root_path;

fn is_installed() -> bool {
    current_exe()
        .unwrap()
        .iter()
        .map(|t| t.to_str().unwrap())
        .collect::<Vec<_>>()
        .contains(&".nuke")
}

fn main() {
    colored::control::set_virtual_terminal(true).unwrap();

    let mut matches = Command::new("Nuke");
    if is_installed() {
        matches = matches.arg_required_else_help(true);
    }
    let matches = matches
        .subcommand(
            Command::new("install")
                // .short_flag('i')
                .about("Install a new version Node.js")
                .arg(Arg::new("VERSION").help("Node.js version").required(true))
                .arg(
                    Arg::new("arch")
                        .value_name("ARCH")
                        .help("Node.js architecture")
                        .default_value("x64"),
                ),
        )
        .subcommand(
            Command::new("default")
                .visible_alias("use")
                .about("Set the default Node.js version")
                .arg(Arg::new("VERSION").help("Node.js version").required(true))
                .arg(
                    Arg::new("arch")
                        .value_name("ARCH")
                        .help("Node.js architecture")
                        .default_value("x64"),
                ),
        )
        .subcommand(Command::new("setup").about("Setup nuke to current logged user"))
        .subcommand(
            Command::new("scan-bin")
                .about("Scan and map the commands in each Node version")
                .arg(Arg::new("silent").long("silent").help("Disable output")),
        )
        .subcommand(Command::new("list").about("List the installed versions of Node.js"))
        .subcommand(Command::new("bin").about("Display the bin directory of Nuke"))
        .get_matches();

    let result = if let Some(ref matches) = matches.subcommand_matches("setup") {
        setup::setup_nuke(Some(matches))
    } else if let Some(ref matches) = matches.subcommand_matches("default") {
        default::set_default_node(matches)
    } else if let Some(ref matches) = matches.subcommand_matches("install") {
        install::install_node(matches)
    } else if let Some(ref matches) = matches.subcommand_matches("scan-bin") {
        scan_bin::scan_bin(Some(matches))
    } else if let Some(_) = matches.subcommand_matches("list") {
        list::list_installed_node()
    } else if let Some(_) = matches.subcommand_matches("bin") {
        println!("{}", root_path().join("bin").to_str().unwrap());
        Ok(())
    } else {
        if Confirm::new()
            .with_prompt("Do you want to install nuke for current user?")
            .interact()
            .unwrap()
        {
            if let Err(e) = setup::setup_nuke(None) {
                println!("{} {}", "[Nuke]".red(), e.to_string());
            }
            let _ = std::process::Command::new("cmd.exe").arg("/c").arg("PAUSE").status();
            Ok(())
        } else {
            Ok(())
        }
    };

    if let Err(e) = result {
        println!("{} {}", "[Nuke]".red(), e.to_string());
    }
}
