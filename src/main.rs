extern crate mmix;

use mmix::cli;
use mmix::machine::Machine;

use std::fs::File;
use std::io::{Error, Read};

fn main() {
    // TODO read filename(s) from command line parameters

    let exec = cli::get_execute();
    let filename = exec.get_filename();


    let bytes = match read_executable(&filename) {
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
        cli::read_string();
    }


}

fn read_executable(name: &str) -> Result<Vec<u8>, Error> {
    File::open(name)?.bytes().collect()
}
