use machine::state::State;

pub fn fsqrt(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[z].into();

    // Execute
    let res: f64 = match y {
        1 => unimplemented!(),
        2 => unimplemented!(),
        3 => unimplemented!(),
        4 => unimplemented!(),
        _ => op1.sqrt(),
    };

    // Store result
    state.gpr[x] = res.into();
}
