use machine::state::State;

/// conditional set if even
pub fn csev(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute
    if op1 % 2 == 0 {
        state.gpr[x] = state.gpr[z];
    }
}
