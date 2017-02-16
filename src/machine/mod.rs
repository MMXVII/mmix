pub mod behavior;
pub mod state;

use self::state::mem::ByteAt;
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

    pub fn load(mut self, pos: u64, data: &[u8]) -> Self {
        for i in (pos..).take(data.len()) {
            self.state.mem[ByteAt(i)] = data[i as usize].into()
        }
        self
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
