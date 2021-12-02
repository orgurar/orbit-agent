extern crate orbit_agent;

use orbit_agent::config;
use std::process;

fn main() {
    // DEBUG: welcome messages
    println!("Started {} v{}", config::NAME, config::VERSION);
    println!(
        "Listening on {}::{}",
        config::server::IP,
        config::server::PORT
    );

    // setup agent
    orbit_agent::setup();

    // run agent
    if let Err(e) = orbit_agent::run() {
        println!("Error has occured, gracefully stopping agent.");
        println!("{}", e);

        process::exit(1);
    }
}
