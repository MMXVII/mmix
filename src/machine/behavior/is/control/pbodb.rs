use machine::state::State;
use machine::behavior::is::control::bodb;

/// probable branch if odd backwards
pub fn pbodb(state: &mut State, x: u8, y: u8, z: u8) {
    bodb(state, x, y, z);
}
