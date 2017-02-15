use machine::state::State;
use machine::behavior::is::memory::stbi;

/// store byte unsigned immediate
pub fn stbui(state: &mut State, x: u8, y: u8, z: u8) {
    stbi(state, x, y, z);
}
