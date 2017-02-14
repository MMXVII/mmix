use machine::state::State;
use machine::state::sr::R;

pub fn puti(state: &mut State, x: u8, _y: u8, z: u8) {
    // Store result
    let sr: R = match x {
         8 | 12 | 15 | 9 | 10 | 11 | 13 | 17 | 18 | 14
         => panic!("Putting isn't allowed"),
         _ => x.into(),
    };
    state.sr[sr] = (z as u64).into()
}
