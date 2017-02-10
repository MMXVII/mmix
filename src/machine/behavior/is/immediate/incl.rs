use machine::state::State;

/// increase by low wyde
pub fn incl(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[x].into();

    // Shift y
    let y = (y as u64) << 8;

    // Execute
    let result = op1.wrapping_add(y + (z as u64));

    // Store result
    state.gpr[x] = result.into();
}
