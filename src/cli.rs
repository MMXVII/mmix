pub struct Config {
    filename: String,
    step: bool,
}

impl Config {
    pub fn parse() -> Self {
        // TODO
        unimplemented!();
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn step(&self) -> bool {
        self.step
    }
}
