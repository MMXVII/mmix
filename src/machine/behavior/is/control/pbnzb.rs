use machine::state::State;
use machine::behavior::is::control::bnzb;

/// /// probable branch if nonzero backwards
pub fn pbnzb(state: &mut State, x: u8, y: u8, z: u8) {
    bnzb(state, x, y, z);
}
