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
        State {
            // for the moment start execution at address 0
            pc: 0,

            sr: SRegisters::new(),
            gpr: GPRegisters::new(),

            // for the moment create 1MB of memory
            mem: Memory::with_capacity(0x100000)
        }
    }
}
