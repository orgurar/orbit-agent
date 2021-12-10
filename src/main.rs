extern crate orbit_agent;

use std::{thread, time::Duration};
use orbit_agent::config::reconnect;

fn main() -> ! {
    // setup agent
    orbit_agent::setup();

    // run agent with a reconnection loop
    loop {
        if let Err(e) = orbit_agent::run() {
            // send error to the server, then start the agent again
            eprintln!("Error has occured, gracefully stopping agent.");
            eprintln!("{}", e);

            // sleep for a given amount of time, the reconnect
            eprintln!("Trying to reconnect in {} seconds...", reconnect::DELAY);
            thread::sleep(Duration::from_millis(reconnect::DELAY));
            continue;
        }
    }
}
