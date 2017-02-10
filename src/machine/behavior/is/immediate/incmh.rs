use machine::state::State;

/// increase by medium high wyde
pub fn incmh(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[x].into();

    // Shift y and z
    let y = (y as u64) << 40;
    let z = (z as u64) << 32;

    // Execute
    let result = op1.wrapping_add(y + z);

    // Store result
    state.gpr[x] = result.into();
}
