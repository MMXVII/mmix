use machine::state::State;

pub fn fun(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[y].into();
    let op2: f64 = state.gpr[z].into();

    // Execute
    let res: i64 = if op1.is_nan() || op1 > op2 {
        1
    } else if op2.is_nan() || op1 < op2 {
        -1
    } else {
        0
    };

    // Store result
    state.gpr[x] = res.into();
}
