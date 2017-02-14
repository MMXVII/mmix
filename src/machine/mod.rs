pub mod behavior;
pub mod state;

use self::state::State;

pub struct Machine {
    state: State,
    /* TODO add more fields... */
}

impl Machine {
    // TODO FYI: Machine uses the builder pattern.

    pub fn new() -> Self {
        Machine {
            state: State::new()
        }
    }

    pub fn load(self, _pos: u64, _data: &[u8]) -> Self {
        // Loads the slice data to the position pos in memory
        unimplemented!();
    }

    pub fn set_breakpoint(self, _bp: u64) -> Self {
        unimplemented!();
    }

    pub fn start(&mut self) {
        unimplemented!();
    }

    pub fn step(&mut self) {
        self.state.debug_output();
        behavior::cu::cycle(&mut self.state);
    }
}
