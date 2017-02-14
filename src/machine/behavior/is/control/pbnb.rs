use machine::state::State;
use machine::behavior::is::control::bnb;

pub fn pbnb(state: &mut State, x: u8, y: u8, z: u8) {
    bnb(state, x, y, z);
}
