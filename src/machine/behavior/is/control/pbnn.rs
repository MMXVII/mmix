use machine::state::State;
use machine::behavior::is::control::bnn;

pub fn pbnn(state: &mut State, x: u8, y: u8, z: u8) {
    bnn(state, x, y, z);
}
