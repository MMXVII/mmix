use machine::state::State;

pub fn fadd(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[y].into();
    let op2: f64 = state.gpr[z].into();

    // Execute
    let res = op1 + op2;

    // Store result
    state.gpr[x] = res.into();
}
