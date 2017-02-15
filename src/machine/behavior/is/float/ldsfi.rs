use machine::state::State;
use machine::state::mem::TetraAt;

pub fn ldsfi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let a = op1.wrapping_add(z as u64);

    // Load from memory
    let res: f32 = state.mem[TetraAt(a)].into();

    // Store result
    state.gpr[x] = (res as f64).into();
}
