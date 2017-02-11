use machine::state::State;
use machine::state::mem::TetraAt;

pub fn ldsf(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    let a = op1.wrapping_add(op2);

    // Load from memory
    let res: f32 = state.mem[TetraAt(a)].into();

    // Store result
    state.gpr[x] = (res as f64).into();
}
