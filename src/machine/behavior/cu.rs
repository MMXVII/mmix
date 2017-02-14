use machine::state::State;
use machine::state::mem::ByteAt;

pub fn cycle(state: &mut State) {

    // The program counter in state may point to any of the four bytes in the
    // tetrabyte that is to be executed next.
    // We compute the address of the first byte of the next instruction, which stores
    // the instruction's opcode
    let opcode_address = state.pc & 0xFFFFFFFFFFFFFF00;

    // Fetch opcode and operands
    let opcode: u8 = state.mem[ByteAt(opcode_address)].into();
    let x: u8 = state.mem[ByteAt(opcode_address + 1)].into();
    let y: u8 = state.mem[ByteAt(opcode_address + 2)].into();
    let z: u8 = state.mem[ByteAt(opcode_address + 3)].into();

    // Get instruction implementation corresponding to opcode
    let instruction = super::is::get_instruction(opcode);

    // Execute instruction
    instruction(state, x, y, z);

    // Increase program counter
    state.pc += 4;
}
