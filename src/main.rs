extern crate mmix;

use mmix::machine::Machine;

use std::fs::File;
use std::io::{Error, Read};

fn main() {
    // TODO read filename(s) from command line parameters
    let filename = "test.mmo";


    let bytes = match read_executable(filename) {
        Ok(vec) => vec,
        _ => {
            println!("Could not read executable!");
            return
        }
    };


    // Instantiate new machine and load the executable starting at address 0
    let mut mmix = Machine::new().load(0, &bytes);


    loop {
        mmix.step();
        read_string();
    }


}

fn read_executable(name: &str) -> Result<Vec<u8>, Error> {
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
