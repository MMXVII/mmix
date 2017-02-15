use machine::state::State;
use machine::behavior::is::control::bn;

pub fn pbn(state: &mut State, x: u8, y: u8, z: u8) {
    bn(state, x, y, z);
}
