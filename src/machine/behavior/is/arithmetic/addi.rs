use machine::state::State;

pub fn addi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute
    let res = (z as i64).wrapping_add(op1);

    // Store result
    state.gpr[x] = res.into();
}
