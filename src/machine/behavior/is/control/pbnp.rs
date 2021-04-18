use machine::state::State;
use machine::behavior::is::control::bnp;

/// probable branch if nonpositive
pub fn pbnp(state: &mut State, x: u8, y: u8, z: u8) {
    bnp(state, x, y, z);
}
