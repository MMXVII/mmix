use machine::state::State;

pub fn cmp(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: i64 = state.gpr[y].into();
    let op2: i64 = state.gpr[z].into();

    // Execute
    let res : i64 = op1.cmp(&op2) as i64;

    // Store result
    state.gpr[x] = res.into();
}
