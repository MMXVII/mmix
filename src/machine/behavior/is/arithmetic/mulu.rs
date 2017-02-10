use machine::state::State;
use machine::state::sr::R;

pub fn mulu(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: u64 = state.gpr[y].into();
    let op2: u64 = state.gpr[z].into();

    /*
     *  Theory:
     *
     *  First, we split the operands into parts ("digits") of 32 bit:
     *
     *      op1 == 2^32 * hi(op1) + lo(op2)
     *      op2 == 2^32 * hi(op2) + lo(op2)
     *
     *  Then, we use the multiplication algorithm from the primary school:
     *
     *      op1 * op2
     *      == (2^32 * hi(op1) + lo(op2)) * (2^32 * hi(op2) + lo(op2))
     *      ==   2^64 * hi(op1) * hi(op2)
     *         + 2^32 * hi(op1) * lo(op2)
     *         + 2^32 * lo(op1) * hi(op2)
     *         +        lo(op1) * lo(op2)
     *
     *  That's it!
     *
     *  Notes:
     *    - Splitting the operands can be done with simple bit operations.
     *    - No multiplication should wrap!
     *    - A multiplication with 2^n is a left-shift for n (binary) digits.
     */

    // Calculate the single parts
    let mut h =   hi(op1) * hi(op2);   // XX__
    let     m =   hi(op1) * lo(op2)    // _XX_
                + lo(op1) * hi(op2);
    let mut l =   lo(op1) * lo(op2);   // __XX

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
