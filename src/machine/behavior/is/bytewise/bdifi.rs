use machine::state::State;

/// byte difference immediate
pub fn bdifi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let mut result = (op1 >> 8) << 8;
    let y_byte = super::get_byte(op1);
    let interim = y_byte.saturating_sub(z) as u64;
    result += interim;

    // Store result
    state.gpr[x] = result.into();
}
