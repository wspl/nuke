use anyhow::{anyhow, Result};
use clap::ArgMatches;
use colored::*;
use std::{env, fs};
use winapi::{
    shared::minwindef::LPARAM,
    um::winuser::{SendMessageW, HWND_BROADCAST, WM_SETTINGCHANGE},
};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS},
    RegKey,
};

pub fn setup_nuke(_: Option<&ArgMatches>) -> Result<()> {
    let home = dirs::home_dir().ok_or(anyhow!("Cannot find user home dictionary"))?;

    let root_dir = home.join(".nuke");
    let _ = fs::create_dir(&root_dir);

    let bin_dir = root_dir.join("bin");
    let _ = fs::create_dir(&bin_dir);

    let versions_dir = root_dir.join("versions");
    let _ = fs::create_dir(&versions_dir);

    let temp_dir = root_dir.join("temp");
    let _ = fs::create_dir(&temp_dir);

    let current_exe = env::current_exe()?;
    let installed_exe = bin_dir.join("nuke.exe");
    fs::copy(current_exe, installed_exe)?;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_env = hkcu.open_subkey_with_flags("Environment", KEY_ALL_ACCESS)?;
    let path: String = reg_env.get_value("Path")?;
    let mut paths = path
        .split(";")
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>();
    let bin_dir_str = bin_dir.to_str().unwrap();
    if !paths.contains(&bin_dir_str) {
        paths.push(bin_dir_str);
        let new_path = format!("{};", paths.join(";"));
        reg_env.set_value("Path", &new_path)?;
        unsafe {
            let mut buf: Vec<u16> = "Environment".encode_utf16().collect();
            buf.push(0);
            SendMessageW(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                0,
                buf.as_mut_ptr() as LPARAM,
            );
        }
    }

    println!("{} Nuke was successfully installed.", "[Nuke]".green());

    Ok(())
}
