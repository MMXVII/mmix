use machine::state::State;
use machine::behavior::is::control::bev;

pub fn pbev(state: &mut State, x: u8, y: u8, z: u8) {
    bev(state, x, y, z);
}
