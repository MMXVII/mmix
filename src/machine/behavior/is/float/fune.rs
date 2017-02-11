use machine::state::State;
use machine::state::sr::R;

pub fn fune(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[y].into();
    let op2: f64 = state.gpr[z].into();
    let eps: f64 = state.sr[R::E].into();

    // Execute
    let res: i64 = if op1.is_nan() || op1 - op2 > eps {
        1
    } else if op2.is_nan() || op1 - op2 < eps {
        -1
    } else {
        0
    };

    // Store result
    state.gpr[x] = res.into();
}
