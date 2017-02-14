use machine::state::types::Octa;

use std::ops::{Index, IndexMut};

pub struct GPRegisters {
    buf: Vec<Octa>,
}

impl GPRegisters {
    pub fn new() -> Self {
        GPRegisters {
            buf: vec![0u64.into(); 256],
        }
    }
}

impl Index<u8> for GPRegisters {
    type Output = Octa;
    fn index(&self, index: u8) -> &Self::Output {
        self.buf.index(index as usize)
    }
}

impl IndexMut<u8> for GPRegisters {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        self.buf.index_mut(index as usize)
    }
}
