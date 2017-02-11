use machine::state::State;
use machine::state::sr::R;

pub fn fcmpe(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[y].into();
    let op2: f64 = state.gpr[z].into();
    let eps: f64 = state.sr[R::E].into();

    // Execute
    let res: i64 = match op1 - op2 {
        d if d.abs() <= eps =>  0,
        d if d > 0.0        =>  1,
        _                   => -1, 
    };

    // Store result
    state.gpr[x] = res.into();
}
