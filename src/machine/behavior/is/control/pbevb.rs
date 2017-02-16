use machine::state::State;
use machine::behavior::is::control::bevb;

/// probable branch if even backwards
pub fn pbevb(state: &mut State, x: u8, y: u8, z: u8) {
    bevb(state, x, y, z);
}
