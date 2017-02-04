use machine::state::State;
use machine::state::sr::R;

pub fn div(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: i64 = state.gpr[y].into();
    let op2: i64 = state.gpr[z].into();

    // Execute and store results
    // check if denominator == 0
    if op2 == 0 {
        state.sr[R::R] = op1.into();
        state.gpr[x] = 0i64.into();
    } else {
        let res = op1.wrapping_div(op2);
        let rem = op1 - res;   // remainder

        state.sr[R::R] = rem.into();
        state.gpr[x] = res.into();
    }
}
