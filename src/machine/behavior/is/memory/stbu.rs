use machine::state::State;
use machine::behavior::is::memory::stb;

/// store byte unsigned
pub fn stbu(state: &mut State, x: u8, y: u8, z: u8) {
    stb(state, x, y, z);
}
