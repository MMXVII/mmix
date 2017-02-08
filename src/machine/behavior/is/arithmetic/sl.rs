use machine::state::State;

pub fn sl(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();
    let op2: i64 = state.gpr[z].into();

    // Execute
    let res = op1.overflowing_shl(op2 as u32).0;

    // Store results
    state.gpr[x] = res.into();
}
