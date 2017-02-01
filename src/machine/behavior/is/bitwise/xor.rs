use machine::state::State;

/// bitwise exclusive-or
pub fn xor(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    let res = op1 ^ op2;

    // Store result
    state.gpr[x] = res.into();
}
