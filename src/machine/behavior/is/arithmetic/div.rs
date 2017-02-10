use machine::state::State;
use machine::state::sr::R;

pub fn div(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: i64 = state.gpr[y].into();
    let op2: i64 = state.gpr[z].into();

    // Execute
    let res;
    let rem; // remainder

    if op2 == 0 {   // special case: denominator == 0
        res = 0;
        rem = op1;
    } else {
        res = op1.wrapping_div(op2);
        rem = op1 - res;
    }

    // Store
    state.gpr[x]   = res.into();
    state.sr[R::R] = rem.into();
}
