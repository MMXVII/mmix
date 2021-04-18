use machine::state::State;
use machine::behavior::is::control::bod;

/// probable branch if odd
pub fn pbod(state: &mut State, x: u8, y: u8, z: u8) {
    bod(state, x, y, z);
}
