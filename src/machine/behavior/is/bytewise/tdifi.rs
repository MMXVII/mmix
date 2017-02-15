use machine::state::State;

/// tetra difference immediate
pub fn tdifi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let mut result = (op1 >> 32) << 32;
    let y_tetra = super::get_tetra(op1);
    let interim = y_tetra.saturating_sub(z as u32) as u64;
    result += interim;

    // Store result
    state.gpr[x] = result.into();
}
