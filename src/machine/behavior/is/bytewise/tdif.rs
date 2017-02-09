use machine::state::State;

/// tetra difference
pub fn tdif(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let mut op1: u64 = state.gpr[y].into();
    let mut op2: u64 = state.gpr[z].into();

    // Execute
    let mut result: u64 = 0;
    for i in 0..2 {
        let y_tetra = get_tetra(op1);
        let z_tetra = get_tetra(op2);
        let interim = (y_tetra.saturating_sub(z_tetra) as u64) << (32 * i);
        result += interim;
        op1 >>= 32;
        op2 >>= 32;
    }

    // Store result
    state.gpr[x] = result.into();
}

fn get_tetra(bits: u64) -> u32 {
    bits as u32
}
