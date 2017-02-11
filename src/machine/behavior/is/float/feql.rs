use machine::state::State;

pub fn feql(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[y].into();
    let op2: f64 = state.gpr[z].into();

    // Execute and store result
    if op1 - op2 == 0.0 {
    	state.gpr[x] = 1i64.into();
    } else {
    	state.gpr[x] = 0i64.into();
    }
}
