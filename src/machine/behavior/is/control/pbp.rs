use machine::state::State;
use machine::behavior::is::control::bp;

/// probable branch if positive backwards
pub fn pbp(state: &mut State, x: u8, y: u8, z: u8) {
    bp(state, x, y, z);
}
