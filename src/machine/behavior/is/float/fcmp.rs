use machine::state::State;

pub fn fcmp(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[y].into();
    let op2: f64 = state.gpr[z].into();

    // Execute
    let res: i64 = match op1 - op2 {
        d if d > 0.0 =>  1,
        d if d < 0.0 => -1,
        _            =>  0,
    };

    // Store result
    state.gpr[x] = res.into();
}
