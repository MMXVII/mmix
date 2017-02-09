use machine::state::State;

/// byte difference
pub fn bdif(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let mut op1: u64 = state.gpr[y].into();
    let mut op2: u64 = state.gpr[z].into();

    // Execute
    let mut result: u64 = 0;
    for i in 0..8 {
        let y_byte = get_byte(op1);
        let z_byte = get_byte(op2);
        let interim = (y_byte.saturating_sub(z_byte) as u64) << (8 * i);
        result += interim;
        op1 >>= 8;
        op2 >>= 8;
    }

    // Store result
    state.gpr[x] = result.into();
}

fn get_byte(bits: u64) -> u8 {
    bits as u8
}
