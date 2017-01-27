pub mod gpr;
pub mod mem;
pub mod sr;
pub mod types;

use self::sr::SRegisters;
use self::gpr::GPRegisters;
use self::mem::Memory;

pub struct State {
    pub pc: u64,
    pub sr: SRegisters,
    pub gpr: GPRegisters,
    pub mem: Memory,
}

impl State {
    pub fn new() -> Self {
        unimplemented!();
    }
}
