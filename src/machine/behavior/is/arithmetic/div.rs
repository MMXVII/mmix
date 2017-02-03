use machine::state::State;
use machine::behavior::is::other::put;

pub fn div(state: &mut State, x: u8, y: u8, z: u8) {
    // load operands
    let op1: i64 = state.gpr[y].into();
    let op2: i64 = state.gpr[z].into();
    // check if denominator == 0
    if op2 == 0 {
        put(state, (op1 as u8), 6, 0);
        state.gpr[x] = (0 as i64).into();
    } else {
        let res = op1 / op2;
        let rem = op1 - res;   // remainder
        put(state, (rem as u8), 6, 0);

        state.gpr[x] = res.into();
    }
}
