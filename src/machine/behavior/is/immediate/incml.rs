use machine::state::State;

/// increase by medium low wyde
pub fn incml(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[x].into();

    // Shift y and z
    let y = (y as u64) << 24;
    let z = (z as u64) << 16;

    // Execute
    let result = op1.wrapping_add(y + z);

    // Store result
    state.gpr[x] = result.into();
}
