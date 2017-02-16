use clap::{App, AppSettings, Arg};

#[derive(Debug)]
pub struct Config {
    filename: String,
    step: bool,
}

impl Config {
    pub fn parse() -> Self {
        // Parse the CLI arguments using clap
        let m = app().get_matches();

        // Return a configuration
        Config {
            filename:   m.value_of("filename").unwrap().into(),
            step:       m.is_present("step"),
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn step(&self) -> bool {
        self.step
    }
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(       "mmixvm"                                                        )
        .version(   "0.1.0"                                                         )
        .about(     "An MMIX virtual machine written in Rust"                       )
        .setting(   AppSettings::ColoredHelp                                        )
        .arg(
            Arg::with_name(         "filename"                                      )
                .value_name(        "filename"                                      )
                .help(              "A binary used to initialize the main memory"   )
                .required(          true                                            )
                .index(             1                                               )
        )
        .arg(
            Arg::with_name(         "step"                                          )
                .help(              "Specifies to execute the program step-by-step" )
                .short(             "s"                                             )
                .long(              "step"                                          )
        )
}
