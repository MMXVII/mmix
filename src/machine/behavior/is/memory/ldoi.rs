use machine::state::State;
use machine::state::mem::OctaAt;

/// load octa immediate
pub fn ldoi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let a = op1.wrapping_add(z as u64);

    // Load from memory
    state.gpr[x] = state.mem[OctaAt(a)];
}
