use machine::state::State;
use machine::behavior::is::control::bnz;

pub fn pbnz(state: &mut State, x: u8, y: u8, z: u8) {
    bnz(state, x, y, z);
}
