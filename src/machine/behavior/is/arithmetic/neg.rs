use machine::state::State;

pub fn neg(state: &mut State, x: u8, y: u8, z: u8) {
    // load operand
    let op2: i64 = state.gpr[z].into();
    // execute
    let res = (y as i64) - op2;
    // store result
    state.gpr[x] = res.into(); 
}
