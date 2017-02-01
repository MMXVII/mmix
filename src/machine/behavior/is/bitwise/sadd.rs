use machine::state::State;

/// sideways add
pub fn sadd(state: &mut State, x: u8, y: u8, z: u8) {
    // calculate andn and load result
    super::andn(state, x, y, z);
    let mut bits: u64 = state.gpr[x].into();
    // Execute
    let mut res: u64 = 0;
    while bits != 0 {
        if bits % 2 != 0 {
            res += 1;
        }
        bits /= 2;
    }
    // Store result
    state.gpr[x] = res.into();
}
