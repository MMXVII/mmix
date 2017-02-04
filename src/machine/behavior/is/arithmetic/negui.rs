use machine::state::State;

pub fn negui(state: &mut State, x: u8, y: u8, z: u8) {
    // Execute
    let res = (y as u64).wrapping_sub(z as u64);

    // Store result
    state.gpr[x] = res.into();
}
