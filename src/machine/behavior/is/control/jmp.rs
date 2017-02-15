use machine::state::State;

pub fn jmp(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let x: u64 = (x as u64) << 16;
    let y: u64 = (y as u64) <<  8;
    let xyz: u64 = x + y + (z as u64);
    let at: u64 = state.pc.into();

    // Execute
    let ra: u64 = at + 4 * xyz;

    // Store result
    state.pc = ra.into();
}
