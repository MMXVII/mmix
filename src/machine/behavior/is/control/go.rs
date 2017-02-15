 use machine::state::State;

pub fn go(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    let res = op1.wrapping_add(op2);

    // Store result
    state.gpr[x] = (state.pc + 4).into();
    state.pc = res.into();
}
