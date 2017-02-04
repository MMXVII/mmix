use machine::state::State;

pub fn muli(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute
    let res = op1.wrapping_mul(z as i64);

    // Store result
    state.gpr[x] = res.into();
}
