use machine::state::State;
use machine::state::mem::TetraAt;

pub fn stsfi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let a = op1.wrapping_add(z as u64);

    // Load x
    let res: f64 = state.gpr[x].into();

    // Store in memory
    state.mem[TetraAt(a)] = (res as f32).into();
}
