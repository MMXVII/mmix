use machine::state::State;

/// zero or set if even
pub fn zsev(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute
    if op1 % 2 == 0 {
        state.gpr[x] = state.gpr[z];
    } else {
        state.gpr[x] = 0u64.into();
    }
}
