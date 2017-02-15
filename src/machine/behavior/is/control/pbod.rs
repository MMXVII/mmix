use machine::state::State;
use machine::behavior::is::control::bod;

pub fn pbod(state: &mut State, x: u8, y: u8, z: u8) {
    bod(state, x, y, z);
}
