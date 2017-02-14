pub mod gpr;
pub mod mem;
pub mod sr;
pub mod types;

use machine::behavior::is::get_instruction_name;
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
            // For the moment start execution at address 0
            pc: 0,

            sr: SRegisters::new(),
            gpr: GPRegisters::new(),

            // For the moment create 1MB of memory
            mem: Memory::with_capacity(0x100000)
        }
    }

    /// Display the state partially on the command line for testing purposes
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
        let name = get_instruction_name(opcode);

        println!("");
        println!("PC: {}", self.pc);
        println!("Next instruction: {} x: #{:02x}, #y: {:02x}, z: #{:02x}", name, x, y, z);
    }
}
