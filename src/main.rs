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
        Machine::new()              // Create   //TODO idea: Machine::with_memory(...)
                .load(0, &exe);     // Load executable

    // Determine the execution method
    let run = match config.step() {
        true  => Machine::step,
        false => Machine::start,
    };

    // Start the machine
    loop {
        run(&mut mmix);
        mmix.debug_output();
        read_string();  // TODO wait for user input, this can be done in a better way
    }
}

fn read(name: &str) -> Result<Vec<u8>, Error> {
    File::open(name)?.bytes().collect()
}

/// Reads a string from the terminal/user.
fn read_string() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}
