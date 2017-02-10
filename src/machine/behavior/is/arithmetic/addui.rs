use machine::state::State;

pub fn addui(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let res = (z as u64).wrapping_add(op1);

    // Store result
    state.gpr[x] = res.into();
}
