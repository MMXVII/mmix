use machine::state::State;
use machine::behavior::is::memory::sto;

/// store octa uncached
pub fn stunc(state: &mut State, x: u8, y: u8, z: u8) {
    sto(state, x, y, z);
}
