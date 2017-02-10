use machine::state::State;
use machine::behavior::is::memory::ldoi;

/// load octa unsigned immediate
pub fn ldoui(state: &mut State, x: u8, y: u8, z: u8) {
    ldoi(state, x, y, z);
}
