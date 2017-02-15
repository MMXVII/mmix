use machine::state::State;
use machine::state::mem::TetraAt;

/// load tetra immediate
pub fn ldti(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let a = op1.wrapping_add(z as u64);

    // Load from memory
    let res: i32 = state.mem[TetraAt(a)].into();

    // Store result
    state.gpr[x] = (res as i64).into();
}
