use machine::state::State;

pub fn fint(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[z].into();

    // Execute
    let mut res: f64;
    match y {
    	1 => res = op1.round(),    // round off
    	2 => res = op1.ceil(),     // round up
    	3 => res = op1.floor(),    // round down
    	4 => {                     // round near to ten
            res = (op1 + 5.0) / 10.0;
            res = res.floor() * 10.0;
        },
    	_ => res = op1.round(),    // round off
    }

    // Store result
    state.gpr[x] = res.into();
}
