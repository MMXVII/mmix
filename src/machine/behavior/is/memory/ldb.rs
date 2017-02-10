use machine::state::State;
use machine::state::mem::ByteAt;

/// load byte
pub fn ldb(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    let a = op1.wrapping_add(op2);

    // Load from memory
    let res: i8 = state.mem[ByteAt(a)].into();

    // Store result
    state.gpr[x] = (res as i64).into();
}
