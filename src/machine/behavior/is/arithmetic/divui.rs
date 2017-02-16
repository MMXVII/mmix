use machine::state::State;
use machine::state::sr::R;

use extprim::u128::{u128, div_rem};

/// divide unsigned
pub fn divui(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let num_high:   u64 = state.sr[R::D].into();
    let num_low:    u64 = state.gpr[y].into();
    let denom           = z as u64;

    // Execute
    if denom != 0 {
        if denom > num_high {

            let num = u128::from_parts(num_high, num_low);
            let (divi, rem) = div_rem(num, u128::new(denom));
            // Store result
            state.gpr[x] = divi.low64().into();
            state.sr[R::R] = rem.low64().into();

        } else {
            // Store results
            state.gpr[x] = state.sr[R::D];
            state.sr[R::R] = state.gpr[y];
        }
    }
}
