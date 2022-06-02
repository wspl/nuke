use std::{env, path::PathBuf};

pub fn root_path() -> PathBuf {
    let mut exe_dir = env::current_exe().unwrap();
    exe_dir.pop();
    if !exe_dir.ends_with(PathBuf::from_iter(&[".nuke", "bin"])) {
        panic!("cannot access root path if uninstalled")
    }
    exe_dir.pop();
    exe_dir
}
