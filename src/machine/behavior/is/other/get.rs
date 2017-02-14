use machine::state::State;
use machine::state::sr::R;

pub fn get(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let r = y.wrapping_add(z);

    // Execute
    if r > 32 {
    	panic!("no register");
    }
    let sr: R = (z as u8).into();
    let res: u64 = state.sr[sr].into();

    // Store result
    state.gpr[x] = res.into();
}
