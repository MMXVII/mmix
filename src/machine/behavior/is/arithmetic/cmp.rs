use machine::state::State;

pub fn cmp(state: &mut State, x: u8, y: u8, z: u8) {
    // load operands
    let op1: i64 = state.gpr[y].into();
    let op2: i64 = state.gpr[z].into();
    // check if op1 is greater, less or equal than op2
    // store result
    if op1 < op2 {
    	state.gpr[x] = (-1 as i64).into();
    } else if op1 > op2 {
    	state.gpr[x] = (1 as i64).into();
    } else {
    	state.gpr[x] = (0 as i64).into();
    }
}
