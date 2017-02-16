extern crate clap;

use self::clap::{App, Arg, ArgMatches, SubCommand};

pub struct Execute {
    filename: String,
    step: bool,
}

impl Execute {

    fn new() -> Self {
        Execute {
            filename: "".to_string(),
            step: false,
        }
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }

    pub fn get_step(&self) -> bool {
        self.step
    }

}

pub fn get_execute() -> Execute {

    let mut exe = Execute::new();
    let mut valid = false;

    while !valid {

        valid = true;

        let params = App::new("params")
            .arg(
                Arg::with_name("mode")
                .long("mode")
                .possible_values(&["run", "step"])
                .default_value("step")
            )
            .arg(
                Arg::with_name("name")
                .long("name")
                .required(true)
                .takes_value(true)
            )
            .get_matches();

        if let Some(name) = params.value_of("name") {
            exe.filename = name.to_string();
            println!("{:?}", exe.filename);
        } else {
            println!("{:?}", params.usage());
            println!("filename");
            valid = false;
            break;
        }

        if let Some(mode) = params.value_of("mode") {
            exe.step = mode == "step";
            println!("{:?}", exe.step);
        } else {
            println!("{:?}", params.usage());
            println!("mode");
            valid = false;
        }
        valid = true;
    }
    exe
}

/// Reads a string from the terminal/user.
pub fn read_string() -> String {
    let mut buffer = String::new();
    ::std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}
