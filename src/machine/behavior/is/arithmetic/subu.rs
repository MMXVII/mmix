use machine::state::State;

pub fn subu(state: &mut State, x: u8, y: u8, z: u8) {
    // load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();
    // execute
    let res = op1.wrapping_sub(op2);
    // store result
    state.gpr[x] = res.into();
}
