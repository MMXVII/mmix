use machine::state::State;

/// conditional set if odd immediate
pub fn csodi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute
    if op1 % 2 == 1 {
        state.gpr[x] = (z as u64).into();
    }
}
