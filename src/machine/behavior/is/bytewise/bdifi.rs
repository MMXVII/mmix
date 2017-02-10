use machine::state::State;
use machine::behavior::is::bytewise::odifi;

/// byte difference immediate
pub fn bdifi(state: &mut State, x: u8, y: u8, z: u8) {
    odifi(state, x, y, z);
}
