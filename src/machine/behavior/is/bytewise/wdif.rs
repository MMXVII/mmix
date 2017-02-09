use machine::state::State;

/// wyde difference
pub fn wdif(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let mut op1: u64 = state.gpr[y].into();
    let mut op2: u64 = state.gpr[z].into();

    // Execute
    let mut result: u64 = 0;
    for i in 0..4 {
        let y_wyde = get_wyde(op1);
        let z_wyde = get_wyde(op2);
        let interim = (y_wyde.saturating_sub(z_wyde) as u64) << (16 * i);
        result += interim;
        op1 >>= 16;
        op2 >>= 16;
    }

    // Store result
    state.gpr[x] = result.into();
}

fn get_wyde(bits: u64) -> u16 {
    bits as u16
}
