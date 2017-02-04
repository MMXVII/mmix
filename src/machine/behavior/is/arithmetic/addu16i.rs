use machine::state::State;

pub fn addu16i(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();

    // Execute
    let res = (op1.wrapping_mul(16)).wrapping_add(z as u64);

    // Store result
    state.gpr[x] = res.into();
}
