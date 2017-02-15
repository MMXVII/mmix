use machine::state::State;
use machine::behavior::is::memory::ldo;

/// load octa unsigned
pub fn ldou(state: &mut State, x: u8, y: u8, z: u8) {
    ldo(state, x, y, z);
}
