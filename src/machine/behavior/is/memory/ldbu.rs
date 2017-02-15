use machine::state::State;
use machine::state::mem::ByteAt;

/// load byte unsigned
pub fn ldbu(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    let a = op1.wrapping_add(op2);

    // Load from memory
    let res: u8 = state.mem[ByteAt(a)].into();

    // Store result
    state.gpr[x] = (res as u64).into();
}
