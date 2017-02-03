use machine::state::State;
use machine::state::sr::R;

pub fn mulu(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    // Execute
    // lower half
    let low_res = op1.wrapping_mul(op2);

    // upper half
    let upp_op1 = op1.wrapping_shl(32);
    let upp_op2 = op2.wrapping_shl(32);
    let upp_res = upp_op1.wrapping_mul(upp_op2);

    // Store results
    state.gpr[x] = low_res.into();
    state.sr[R::H] = upp_res.into();
}
