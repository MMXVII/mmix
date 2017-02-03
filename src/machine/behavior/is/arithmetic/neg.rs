use machine::state::State;

pub fn neg(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op2: i64 = state.gpr[z].into();

    // Execute
    let res = (y as i64).wrapping_sub(op2);

    // Store result
    state.gpr[x] = res.into();
}
