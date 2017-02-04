use machine::state::State;

pub fn cmpi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute
    let res: i64 = (z as i64).cmp(&op1) as i64;

    // Store result
    state.gpr[x] = res.into();
}
