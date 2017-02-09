use machine::state::State;
use machine::state::sr::R;

pub fn mului(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: u64 = state.gpr[y].into();

    /*
     *  Theory:
     *
     *  See instruction MULU in machine::behavior::is::arithmetic::mulu!
     */

    // Calculate the single parts
    let mut h =  hi(op1) * hi(z as u64);
    let     m =  hi(op1) * lo(z as u64)
               + lo(op1) * hi(z as u64);
    let mut l =  lo(op1) * lo(z as u64);

    // Merge the single parts
    h += hi(m);
    l += lo(m) << 32;

    // Store results
    state.sr[R::H] = h.into();
    state.gpr[x]   = l.into();
}

fn hi(x: u64) -> u64 {
    x >> 32
}

fn lo(x: u64) -> u64 {
    (x as u32) as u64
}
