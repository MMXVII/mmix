use machine::state::State;

/// wyde difference immediate
pub fn wdifi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let mut result = (op1 >> 16) << 16;
    let y_wyde = super::get_wyde(op1);
    let interim = y_wyde.saturating_sub(z as u16) as u64;
    result += interim;

    // Store result
    state.gpr[x] = result.into();
}
