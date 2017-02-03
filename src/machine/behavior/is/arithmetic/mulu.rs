use machine::state::State;
use machine::behavior::is::other::put;

pub fn mulu(state: &mut State, x: u8, y: u8, z: u8) {
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    let res = op1.overflowing_mul(op2);

    if res.1 == false {
        state.gpr[x] = res.0.into();
    } else {
        state.gpr[x] = res.0.into();
        let rh: u8 = (2 as u8).pow(64).wrapping_sub(op1 as u8);
        put(state, rh, 3, 0); // himult register
    }
}

