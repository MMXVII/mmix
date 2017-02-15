use machine::state::State;
use machine::behavior::is::memory::stwi;

/// store wyde unsigned immediate
pub fn stwui(state: &mut State, x: u8, y: u8, z: u8) {
    stwi(state, x, y, z);
}
