use machine::state::State;
use machine::state::sr::R;

pub fn put(state: &mut State, x: u8, _y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[z].into();

    // Store result
    let sr: R = match x {
         8 | 12 | 15 | 9 | 10 | 11 | 13 | 17 | 18 | 14
         => panic!("Putting isn't allowed"),
         _ => x.into(),
    };
    state.sr[sr] = op1.into()
}
