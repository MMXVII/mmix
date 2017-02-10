use machine::state::State;
use machine::state::sr::R;

pub fn divi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute
    let res;
    let rem;

    if z == 0 {
        res = 0;
        rem = op1;
    } else {
        res = op1.wrapping_div(z as i64);
        rem = op1 - res;
    }

    // Store
    state.gpr[x]   = res.into();
    state.sr[R::R] = rem.into();
}
