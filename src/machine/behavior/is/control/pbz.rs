use machine::state::State;
use machine::behavior::is::control::bz;

pub fn pbz(state: &mut State, x: u8, y: u8, z: u8) {
    bz(state, x, y, z);
}
