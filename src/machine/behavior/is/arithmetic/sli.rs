use machine::state::State;

pub fn sli(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute
    let res = op1.overflowing_shl(z as u32).0;

    // Store result
    state.gpr[x] = res.into();
}
