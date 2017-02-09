use machine::state::types::*;

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct ByteAt(pub u64);

#[derive(Copy, Clone)]
pub struct WydeAt(pub u64);

#[derive(Copy, Clone)]
pub struct TetraAt(pub u64);

#[derive(Copy, Clone)]
pub struct OctaAt(pub u64);

pub struct Memory {
    buf: Box<[Octa]>,
}

impl Memory {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn with_capacity(capacity: u64) -> Self {
        // Create an "array" that represents the memory
        let len = (capacity / 8) as usize;
        let buf = Vec::with_capacity(len).into_boxed_slice();

        // Build and return the memory
        Memory {
            buf: buf,
        }
    }
}

impl Index<ByteAt> for Memory {
    type Output = Byte;
    fn index(&self, _: ByteAt) -> &Self::Output {
        unimplemented!();
    }
}

impl IndexMut<ByteAt> for Memory {
    fn index_mut(&mut self, _: ByteAt) -> &mut Self::Output {
        unimplemented!();
    }
}

impl Index<WydeAt> for Memory {
    type Output = Wyde;
    fn index(&self, _: WydeAt) -> &Self::Output {
        unimplemented!();
    }
}

impl IndexMut<WydeAt> for Memory {
    fn index_mut(&mut self, _: WydeAt) -> &mut Self::Output {
        unimplemented!();
    }
}

impl Index<TetraAt> for Memory {
    type Output = Tetra;
    fn index(&self, idx: TetraAt) -> &Self::Output {
        // Find the octa that holds the tetra
        let octa: *const Octa = self.index(OctaAt(idx.0));

        // Calculate the tetra's position within that octa
        let mut pos = (idx.0 % 8 / 4) as isize;
        if cfg!(target_endian = "little") {
            pos = (4 - 1) - pos;
        }

        // Calculate a pointer to the tetra
        let mut tetra = octa as *const Tetra;
        tetra = unsafe {
            tetra.offset(pos)
        };

        // Return the pointer as reference
        unsafe {
            tetra.as_ref()
        }.unwrap()
    }
}

impl IndexMut<TetraAt> for Memory {
    fn index_mut(&mut self, _: TetraAt) -> &mut Self::Output {
        unimplemented!();
    }
}

impl Index<OctaAt> for Memory {
    type Output = Octa;
    fn index(&self, idx: OctaAt) -> &Self::Output {
        let pos = (idx.0 / 8) as usize;
        self.buf.index(pos)
    }
}

impl IndexMut<OctaAt> for Memory {
    fn index_mut(&mut self, idx: OctaAt) -> &mut Self::Output {
        let pos = (idx.0 / 8) as usize;
        self.buf.index_mut(pos)
    }
}
