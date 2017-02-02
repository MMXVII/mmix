use machine::state::State;

pub fn negu(state: &mut State, x: u8, y: u8, z: u8) {
    // load operand
    let op2: u64 = state.gpr[z].into();
    // execute
    let res = (y as u64).wrapping_sub(op2);
    // store result
    state.gpr[x] = res.into();
}
