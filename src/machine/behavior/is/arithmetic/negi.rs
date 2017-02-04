use machine::state::State;

pub fn negi(state: &mut State, x: u8, y: u8, z: u8) {
    // Execute
    let res = (y as i64).wrapping_sub(z as i64);

    // Store result
    state.gpr[x] = res.into();
}
