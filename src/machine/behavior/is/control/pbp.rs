use machine::state::State;
use machine::behavior::is::control::bp;

pub fn pbp(state: &mut State, x: u8, y: u8, z: u8) {
    bp(state, x, y, z);
}
