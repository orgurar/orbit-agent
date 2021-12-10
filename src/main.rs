extern crate orbit_agent;

use std::{thread, time::Duration};
use orbit_agent::config::reconnect;

fn main() -> ! {
    // setup agent
    orbit_agent::setup();

    // run agent with a reconnection loop
    loop {
        if let Err(e) = orbit_agent::run() {
            // recieve the error, and print if in debug mode
            if cfg!(debug_assertions) {
                eprintln!("Error has occured, gracefully stopping agent.");
                eprintln!("{}", e);
            }

            // sleep for a given amount of time, the reconnect
            #[cfg(debug_assertions)]
            eprintln!("Trying to reconnect in {} seconds...", reconnect::DELAY);
            
            thread::sleep(Duration::from_millis(reconnect::DELAY));
            continue;
        }
    }
}
