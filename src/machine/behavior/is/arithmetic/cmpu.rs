use machine::state::State;

pub fn cmpu(state: &mut State, x: u8, y: u8, z: u8) {
    // load operands
    let op1: u64 = state.gpr[y].into();
	let op2: u64 = state.gpr[z].into();
	// execute 
    let dif = op1.wrapping_add(op2);
    // store result
    if x > 0 {
    	state.gpr[x] = (1 as u64).into();
    } else if x == 0 {
    	state.gpr[x] = (0 as u64).into();
    } else {
    	state.gpr[x] = (-1 as i64).into();
    }
}
