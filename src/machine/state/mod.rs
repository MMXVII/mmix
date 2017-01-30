//! Encapsulates the state of an MMIX virtual machine.
//!
//! This module provides algorithms and data structures that implement the state of an MMIX virtual
//! machine. This includes an implementation for
//!
//!   - the main memory (`mem`),
//!   - the general purpose registers (`gpr`) and
//!   - the special registers (`sr`).
//!
//! Details can be found in the submodule documentation. (The name of the submodule is denoted
//! within the brackets!)
//!
//! This module does also contain a struct called `State` which represents the whole state of an
//! MMIX. It provides functions working on a higher level.

pub mod gpr;
pub mod mem;
pub mod sr;
pub mod types;

use machine::behavior::is::get_symbolics;
use self::sr::SRegisters;
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
        // Display content of first 5 GPRegs
        for i in 0 .. 5 {
            let val: u64 = self.gpr[i as u8].into();

            println!("GP[{}]: {:016x}", i, val);
        }

        println!("");

        // Display memory cells 100 - 139 -> 5 Octas
        for i in 0 .. 5 {
            // Compute the index of the octa
            let idx = 100 + 8 * i;

            // Extract value of each wyde contained in octa
            let hi: u16 = self.mem[WydeAt(idx)].into();
            let med_hi: u16 = self.mem[WydeAt(idx + 2)].into();
            let med_lo: u16 = self.mem[WydeAt(idx + 4)].into();
            let lo: u16 = self.mem[WydeAt(idx + 6)].into();
            println!("M[OctaAt({})]: # {:04x} {:04x} {:04x} {:04x}", idx, hi, med_hi, med_lo, lo);
        }


        // Inform about instruction that will be executed in the next cycle
        let opcode: u8 = self.mem[ByteAt(self.pc)].into();
        let x: u8 = self.mem[ByteAt(self.pc + 1)].into();
        let y: u8 = self.mem[ByteAt(self.pc + 2)].into();
        let z: u8 = self.mem[ByteAt(self.pc + 3)].into();
        let name = get_symbolics(opcode);

        println!("");
        println!("PC: {}", self.pc);
        println!("Next instruction: {} x: #{:02x}, y: #{:02x}, z: #{:02x}", name, x, y, z);
    }
}
