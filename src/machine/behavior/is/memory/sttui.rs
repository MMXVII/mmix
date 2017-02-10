use machine::state::State;
use machine::behavior::is::memory::stti;

/// store tetra unsigned immediate
pub fn sttui(state: &mut State, x: u8, y: u8, z: u8) {
    stti(state, x, y, z);
}
