use machine::state::State;
use machine::state::sr::R;

pub fn get(state: &mut State, x: u8, _y: u8, z: u8) {
    // Execute
    let sr: R = (z as u8).into();
    let res: u64 = state.sr[sr].into();

    // Store result
    state.gpr[x] = res.into();
}
