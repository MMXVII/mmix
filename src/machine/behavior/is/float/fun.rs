use machine::state::State;
use machine::behavior::is::float::fcmp;

pub fn fun(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[y].into();
    let op2: f64 = state.gpr[z].into();

    // Execute
    if op1.is_nan() && op2.is_nan() {
    	state.gpr[x] = 0i64.into();
    } else if op1.is_nan() {
    	state.gpr[x] = 1i64.into();
    } else if op2.is_nan() {
    	state.gpr[x] = (-1 as i64).into();
    } else {
    	fcmp(state, x, y, z);
    }
}
