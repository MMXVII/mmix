use machine::state::State;
use machine::state::mem::OctaAt;

/// store octa immediate
pub fn stoi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let a = op1.wrapping_add(z as u64);

    // Store in memory
    state.mem[OctaAt(a)] = state.gpr[x];
}
