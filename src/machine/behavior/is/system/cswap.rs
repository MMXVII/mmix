use machine::state::State;
use machine::state::mem::OctaAt;
use machine::state::sr::R;

/// compare and swap ocatbytes
pub fn cswap(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    let a = op1.wrapping_add(op2);

    if state.mem[OctaAt(a)] == state.sr[R::P] {
        state.mem[OctaAt(a)] = state.gpr[x];
    } else {
        state.sr[R::P] = state.mem[OctaAt(a)];
        state.gpr[x] = 0u64.into();
    }
}
