pub mod generic;
pub mod gpr;
pub mod mem;
pub mod sr;

use self::sr::SRegisters;
use self::gpr::GPRegisters;
use self::mem::Memory;

pub struct State {
    pc: u64,
    sr: SRegisters,
    gpr: GPRegisters,
    mem: Memory,
}

impl State {
    fn new() -> Self {
        unimplemented!();
    }
}
