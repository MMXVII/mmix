use machine::state::State;
use machine::state::sr::R;

pub fn divi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute and store results
    if z == 0 {
    	state.sr[R::R] = op1.into();
    	state.gpr[x] = 0i64.into();
    } else {
    	let res = op1.wrapping_div(z as i64);
    	let rem = op1 - res;

    	state.sr[R::R] = rem.into();
    	state.gpr[x] = res.into();
    }
}
