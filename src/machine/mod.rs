//! Implements an MMIX virtual machine.
//!
//! This module combines any functionality that is necessary to implement an MMIX virtual machine.
//! It provides a struct called `Machine` that is used to assemble one. See the examples below and
//! the struct's documentation to learn how that is done.
//!
//! Components that implement the behavior of an MMIX, e.g. the instruction set, are grouped in a
//! submodule called `behavior`. Components that implement the state, e.g. the main memory or the
//! special and general purpose registers, are grouped in a submodule called `state`.
//!
//! # Examples
//! ```
//! // TODO write an example
//! ```

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
        behavior::cu::cycle(&mut self.state);
    }

    /// Display the state partially on the command line for testing purposes
    /// Be warned, this is only a provisional function, and should be removed later.
    /// Its functionality should be provided by a seperate View struct! FIXME
    pub fn debug_output(&self) {
        self.state.debug_output();
    }
}
