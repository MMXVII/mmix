use machine::state::State;

pub fn sru(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    let res = op1.overflowing_shr(op2 as u32).0;

    // Store results
    state.gpr[x] = res.into();
}
