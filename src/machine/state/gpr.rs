use prelude::*;

use std::ops::{Index, IndexMut};

pub struct GPRegisters(());

impl Index<u8> for GPRegisters {
    type Output = Octa;
    fn index(&self, _: u8) -> &Self::Output {
        unimplemented!();
    }
}

impl IndexMut<u8> for GPRegisters {
    fn index_mut(&mut self, _: u8) -> &mut Self::Output {
        unimplemented!();
    }
}
