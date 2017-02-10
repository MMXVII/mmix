use machine::state::State;
use machine::state::mem::OctaAt;

/// store constant octabyte immediate
pub fn stcoi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let a = op1.wrapping_add(z as u64);

    // Store in memory
    state.mem[OctaAt(a)] = (x as u64).into();
}
