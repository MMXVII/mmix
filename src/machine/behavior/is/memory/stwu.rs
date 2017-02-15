use machine::state::State;
use machine::behavior::is::memory::stw;

/// store wyde unsigned
pub fn stwu(state: &mut State, x: u8, y: u8, z: u8) {
    stw(state, x, y, z);
}
