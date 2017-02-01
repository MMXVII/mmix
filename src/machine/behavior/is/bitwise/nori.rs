use machine::state::State;

/// bitwise not-or immediate
pub fn nori(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let res = !(op1 | z as u64);

    // Store result
    state.gpr[x] = res.into();
}
