use machine::state::State;

/// wyde difference immediate
pub fn wdifi(state: &mut State, x: u8, y: u8, z: u8) {
    odifi(x, y, z);
}
