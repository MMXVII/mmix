use machine::state::State;
use machine::state::mem::OctaAt;
use machine::state::sr::R;

/// compare and swap ocatbytes immediate
pub fn cswapi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let a = op1.wrapping_add(z as u64);

    if state.mem[OctaAt(a)] == state.sr[R::P] {
        state.mem[OctaAt(a)] = state.gpr[x];
    } else {
        state.sr[R::P] = state.mem[OctaAt(a)];
        state.gpr[x] = 0u64.into();
    }
}
