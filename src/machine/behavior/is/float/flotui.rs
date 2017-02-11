use machine::state::State;

pub fn flotui(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: u64 = z as u64;

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

