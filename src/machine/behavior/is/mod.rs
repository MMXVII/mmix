pub mod arithmetic;
pub mod bitwise;
pub mod bytewise;
pub mod conditional;
pub mod control;
pub mod float;
pub mod immediate;
pub mod interrupt;
pub mod memory;
pub mod other;
pub mod subroutine;
pub mod system;

use machine::state::State;

pub fn get(_opcode: u8) -> Semantic {
    unimplemented!();
    // SEMANTICS[opcode]
}

type Semantic = fn(&mut State, u8, u8, u8);
/*
pub const SEMANTICS: [Semantic; 256] = [
                        // symbol   hex  dec

    interrupt::trap,    // TRAP     #00    0
    float::fcmp,        // FCMP     #01    1

    /* TODO */          // PUSHGOI  #BF  191

    other::swym         // SWYM     #FF  255
];
*/
