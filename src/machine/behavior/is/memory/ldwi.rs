use machine::state::State;
use machine::state::mem::WydeAt;

/// load wyde immediate
pub fn ldwi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let a = op1.wrapping_add(z as u64);

    // Load from memory
    let res: i16 = state.mem[WydeAt(a)].into();

    // Store result
    state.gpr[x] = (res as i64).into();
}
