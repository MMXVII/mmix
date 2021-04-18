use machine::state::State;
use machine::behavior::is::control::bzb;

/// probable branch if zero backwards
pub fn pbzb(state: &mut State, x: u8, y: u8, z: u8) {
    bzb(state, x, y, z);
}
