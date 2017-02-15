use machine::state::State;

/// set medium high wyde
pub fn setmh(state: &mut State, x: u8, y: u8, z: u8) {
    // Shift y and z
    let y = (y as u64) << 40;
    let z = (z as u64) << 32;

    // Execute
    let result = y + z;

    // Store result
    state.gpr[x] = result.into();
}
