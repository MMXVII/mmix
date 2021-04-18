use machine::state::State;
use machine::behavior::is::control::bnnb;

/// probable branch if nonnegative backwards
pub fn pbnnb(state: &mut State, x: u8, y: u8, z: u8) {
    bnnb(state, x, y, z);
}
