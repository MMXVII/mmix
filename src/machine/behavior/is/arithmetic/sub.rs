use machine::state::State;

pub fn sub(state: &mut State, x: u8, y: u8, z: u8) {
    // load operands
    let op1: i64 = state.gpr[y].into();
    let op2: i64 = state.gpr[z].into();
    // execute
    let res = op1 - op2;
    // store result
    state.gpr[x] = res.into();
}
