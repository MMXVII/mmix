use machine::state::State;

/// set medium low wyde
pub fn setml(state: &mut State, x: u8, y: u8, z: u8) {
    // Shift y and z
    let y = (y as u64) << 24;
    let z = (z as u64) << 16;

    // Execute
    let result = y + z;

    // Store result
    state.gpr[x] = result.into();
}
