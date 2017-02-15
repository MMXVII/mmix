use machine::state::State;
use machine::behavior::is::control::bpb;

pub fn pbpb(state: &mut State, x: u8, y: u8, z: u8) {
    bpb(state, x, y, z);
}
