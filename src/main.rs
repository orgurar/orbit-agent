extern crate orbit_agent;

use std::process;

fn main() {
    // setup agent
    orbit_agent::setup();

    // run agent
    if let Err(e) = orbit_agent::run() {
        // send error to the server, then start the agent again
        println!("Error has occured, gracefully stopping agent.");
        println!("{}", e);

        process::exit(1);
    }
}
