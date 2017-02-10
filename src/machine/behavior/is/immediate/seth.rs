use machine::state::State;

/// set high wyde
pub fn seth(state: &mut State, x: u8, y: u8, z: u8) {
    // Shift y and z
    let y = (y as u64) << 56;
    let z = (z as u64) << 48;

    // Execute
    let result = y + z;

    // Store result
    state.gpr[x] = result.into();
}
