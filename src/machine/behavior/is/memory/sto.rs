use machine::state::State;
use machine::state::mem::OctaAt;

/// store octa
pub fn sto(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    let a = op1.wrapping_add(op2);

    // Store in memory
    state.mem[OctaAt(a)] = state.gpr[x];
}
