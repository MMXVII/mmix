use machine::state::State;
use machine::state::sr::R;

/// bitwise multiplex
pub fn mux(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();
    let op3: u64 = state.sr[R::M].into();

    // Execute
    let res = (op1 & op3) | (op2 & !op3);

    // Store result
    state.gpr[x] = res.into();
}
