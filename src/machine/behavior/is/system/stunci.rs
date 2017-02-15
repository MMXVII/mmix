use machine::state::State;
use machine::behavior::is::memory::stoi;

/// store octa uncached immediate
pub fn stunci(state: &mut State, x: u8, y: u8, z: u8) {
    stoi(state, x, y, z);
}
