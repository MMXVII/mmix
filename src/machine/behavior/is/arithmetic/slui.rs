use machine::state::State;

pub fn slui(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let res = op1.wrapping_shl(z as u32);

    // Store result
    state.gpr[x] = res.into();
}

