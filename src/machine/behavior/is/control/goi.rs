use machine::state::State;

pub fn goi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();

    // Execute
    let res = op1.wrapping_add(z as u64);

    // Store result
    state.gpr[x] = (state.pc + 4).into();
    state.pc = res.into();
}
