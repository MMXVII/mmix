use machine::state::State;
use machine::behavior::is::control::bnpb;

pub fn pbnpb(state: &mut State, x: u8, y: u8, z: u8) {
    bnpb(state, x, y, z);
}
