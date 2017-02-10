use machine::state::State;
use machine::behavior::is::memory::stt;

/// store tetra unsigned
pub fn sttu(state: &mut State, x: u8, y: u8, z: u8) {
    stt(state, x, y, z);
}
