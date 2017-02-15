use machine::state::State;
use machine::state::mem::TetraAt;

/// load high tetra immediate
pub fn ldhti(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let a = op1.wrapping_add(z as u64);

    // Load from memory
    let res: u32 = state.mem[TetraAt(a)].into();

    // Store result
    state.gpr[x] = ((res as u64) << 32).into();
}
