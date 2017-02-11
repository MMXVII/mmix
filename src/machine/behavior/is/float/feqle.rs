use machine::state::State;
use machine::state::sr::R;

pub fn feqle(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[y].into();
    let op2: f64 = state.gpr[z].into();
    let eps: f64 = state.sr[R::E].into();

    // Execute
    let gap = (op1 - op2).abs();
    let res = (gap <= eps) as i64;

    // Store result
    state.gpr[x] = res.into();
}
