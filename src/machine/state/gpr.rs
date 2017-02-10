use machine::state::types::Octa;

use std::ops::{Index, IndexMut};

pub struct GPRegisters(Vec<Octa>);

impl GPRegisters {
    pub fn new() -> Self {
        GPRegisters(vec![0u64.into(); 256])
    }
}

impl Index<u8> for GPRegisters {
    type Output = Octa;
    fn index(&self, index: u8) -> &Self::Output {
        self.0.index(index as usize)
    }
}

impl IndexMut<u8> for GPRegisters {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        self.0.index_mut(index as usize)
    }
}
