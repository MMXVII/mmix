use machine::state::State;

/// byte difference immediate
pub fn bdifi(state: &mut State, x: u8, y: u8, z: u8) {
    odifi(x, y, z);
}
