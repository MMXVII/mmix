use machine::state::State;
use machine::state::mem::TetraAt;

/// store high tetra immediate
pub fn sthti(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let a = op1.wrapping_add(z as u64);

    // Load x
    let res: i64 = state.gpr[x].into();

    // Store in memory
    state.mem[TetraAt(a)] = ((res >> 32) as i32).into();
}
