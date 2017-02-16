pub mod gpr;
pub mod mem;
pub mod sr;
pub mod types;

use machine::behavior::is::get_symbolics;
use self::sr::{R, SRegisters};
use self::gpr::GPRegisters;
use self::mem::*;

pub struct State {
    pub pc: u64,
    pub sr: SRegisters,
    pub gpr: GPRegisters,
    pub mem: Memory,
}

impl State {
    pub fn new() -> Self {
        State {
            pc:     0,                      // default bootstrap address: 0x0
            sr:     SRegisters::new(),
            gpr:    GPRegisters::new(),
            mem:    Memory::new(),
        }
    }

    pub fn from_parts(pc: u64, sr: SRegisters, gpr: GPRegisters, mem: Memory) -> Self {
        State {
            pc:     pc,
            sr:     sr,
            gpr:    gpr,
            mem:    mem,
        }
    }

    /// Display the state partially on the command line for testing purposes
    /// Be warned, this is only a provisional function, and should be removed later.
    /// Its functionality should be provided by a seperate View struct! FIXME
    pub fn debug_output(&self) {
        // General purpose registers
        for i in 0..5u8 {
            let val: u64 = self.gpr[i].into();

            let hi = (val >> 48) as u16;
            let mh = (val >> 32) as u16;
            let ml = (val >> 16) as u16;
            let lo =  val        as u16;

            println!("gpr[{}]:\t\t{:04x} {:04x} {:04x} {:04x}", i, hi, mh, ml, lo);
        }
        println!();

        // Special registers
        {
            let val: u64 = self.sr[R::R].into();

            let hi = (val >> 48) as u16;
            let mh = (val >> 32) as u16;
            let ml = (val >> 16) as u16;
            let lo =  val        as u16;

            println!("sr[R::R]:\t{:04x} {:04x} {:04x} {:04x}", hi, mh, ml, lo);
        }
        println!("");

        // Memory
        for i in 0..5 {
            let hi: u16 = self.mem[WydeAt(i    )].into();
            let mh: u16 = self.mem[WydeAt(i + 2)].into();
            let ml: u16 = self.mem[WydeAt(i + 4)].into();
            let lo: u16 = self.mem[WydeAt(i + 6)].into();
            println!("mem[OctaAt({})]:\t{:04x} {:04x} {:04x} {:04x}", i, hi, mh, ml, lo);
        }
        println!("");

        // Programm counter
        {
            println!("pc:\t{}", self.pc);

            let op: u8 = self.mem[ByteAt(self.pc    )].into();
            let x:  u8 = self.mem[ByteAt(self.pc + 1)].into();
            let y:  u8 = self.mem[ByteAt(self.pc + 2)].into();
            let z:  u8 = self.mem[ByteAt(self.pc + 3)].into();
            let sym = get_symbolics(op);

            println!("next:\t{} x:{:02x} y:{:02x} z:{:02x}", sym, x, y, z);
        }
        println!("");
        println!("-----------------------------------");
    }
}
