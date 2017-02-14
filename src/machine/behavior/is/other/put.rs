use machine::state::State;
use machine::state::sr::R;

pub fn put(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    if x > 31 {
    	panic!("no register");
    }
    let res = op1.wrapping_add(op2);
    let sr: R = x.into();

    // Store result
    state.sr[sr] = res.into()
}
