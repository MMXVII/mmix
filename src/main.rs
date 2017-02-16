extern crate mmix;

use mmix::cli::Config;
use mmix::machine::Machine;

use std::fs::File;
use std::io::{Error, Read};

fn main() {
    // Parse CLI arguments
    let config = Config::parse();

    // Read the executable
    let exe = match read(config.filename()) {
        Ok(x) => x,
        Err(err) => {
            println!("Cannot read '{}': {}", config.filename(), err);
            return
        }
    };

    // Build a new machine
    let mut mmix =
        Machine::new()              // Create
                .load(0, &exe);     // Load executable

    // Determine the execution method
    let run = match config.step() {
        true  => Machine::step,
        false => Machine::start,
    };

    // Start the machine
    loop {
        mmix.debug_output();    // Print the current state
        pause();                // Wait for the user to press enter
        run(&mut mmix);         // Run the machine until it pauses
    }
}

fn read(name: &str) -> Result<Vec<u8>, Error> {
    File::open(name)?.bytes().collect()
}

fn pause() {
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("something went horribly wrong...");
}
