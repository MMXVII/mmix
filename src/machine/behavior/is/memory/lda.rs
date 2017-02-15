use machine::state::State;
use machine::behavior::is::arithmetic::addu;

/// load address
pub fn lda(state: &mut State, x: u8, y: u8, z: u8) {
    addu(state, x, y, z);
}
