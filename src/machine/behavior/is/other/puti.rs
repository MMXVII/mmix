use machine::state::State;
use machine::state::sr::R;

pub fn puti(state: &mut State, x: u8, y: u8, z: u8) {
    // Execute
    if x > 31 {
    	panic!("no register");
    }
    let res = (y as u64).wrapping_add(z as u64);
    let sr: R = x.into();

    // Store result
    state.sr[sr] = res.into()
}
