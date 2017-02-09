use machine::state::State;

/// octa difference
pub fn odif(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    let result = op1.saturating_sub(op2);

    // Store result
    state.gpr[x] = result.into();
}
