use machine::state::State;

pub fn flot(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[z].into();

    // Execute
    let mut res = op1 as f64;
    match y {
        1 => unimplemented!(),
        2 => unimplemented!(),
        3 => unimplemented!(),
        4 => unimplemented!(),
        _ => res = res,
    }

    // Store result
    state.gpr[x] = res.into();
}
