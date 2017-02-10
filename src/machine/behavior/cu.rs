use machine::state::State;
use machine::state::mem::ByteAt;

pub fn cycle(state: &mut State) {

    assert_eq!(state.pc % 4, 0);

    // fetch opcode and operands
    let opcode: u8 = state.mem[ByteAt(state.pc)].into();
    let x: u8 = state.mem[ByteAt(state.pc + 1)].into();
    let y: u8 = state.mem[ByteAt(state.pc + 2)].into();
    let z: u8 = state.mem[ByteAt(state.pc + 3)].into();

    // get instruction implementation corresponding to opcode
    let instruction = super::is::get(opcode);


    // execute instruction
    instruction(state, x, y, z);


    // increase program counter
    state.pc += 4;

}
