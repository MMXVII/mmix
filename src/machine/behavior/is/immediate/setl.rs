use machine::state::State;

/// set low wyde
pub fn setl(state: &mut State, x: u8, y: u8, z: u8) {
    // Shift y
    let y = (y as u64) << 8;

    // Execute
    let result = y + (z as u64);

    // Store result
    state.gpr[x] = result.into();
}
