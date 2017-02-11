use machine::state::State;
use machine::state::mem::TetraAt;

pub fn stsf(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[y].into();
    let op2: f64 = state.gpr[z].into();

    // Execute
    let a = op1.wrapping_add(op2);

    // Load x
    let res: f64 = state.gpr[x].into();

    // Store in memory
    state.mem[TetraAt(a)] = (res as f32).into();
}
