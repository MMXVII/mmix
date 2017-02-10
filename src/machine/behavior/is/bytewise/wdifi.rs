use machine::state::State;
use machine::behavior::is::bytewise::odifi;

/// wyde difference immediate
pub fn wdifi(state: &mut State, x: u8, y: u8, z: u8) {
    odifi(state, x, y, z);
}
