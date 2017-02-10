use machine::state::State;
use machine::behavior::is::bytewise::odifi;

/// tetra difference immediate
pub fn tdifi(state: &mut State, x: u8, y: u8, z: u8) {
    odifi(state, x, y, z);
}
