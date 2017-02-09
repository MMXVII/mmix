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
    buf: Vec<Octa>,
}

impl Memory {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn with_capacity(bytes: u64) -> Self {
        // Compute the number of octas needed
        let octas = ((bytes + 7) / 8) as usize;

        // Create a zero initialized vector that represents the memory
        let buf = vec![0u64.into(); octas];

        // Build and return the memory
        Memory {
            buf: buf,
        }
    }
}

impl Index<ByteAt> for Memory {
    type Output = Byte;
    fn index(&self, idx: ByteAt) -> &Self::Output {
        // Find the octa that holds the byte
        let octa: *const Octa = self.index(OctaAt(idx.0));

        // Calculate the byte's position within that octa
        let mut pos = (idx.0 % 8 / 1) as isize;
        if cfg!(target_endian = "little") {
            pos = (8 - 1) - pos;
        }

        // Calculate a pointer to the byte
        let mut byte = octa as *const Byte;
        byte = unsafe {
            byte.offset(pos)
        };

        // Return the pointer as reference
        unsafe {
            byte.as_ref()
        }.unwrap()
    }
}

impl IndexMut<ByteAt> for Memory {
    fn index_mut(&mut self, idx: ByteAt) -> &mut Self::Output {
        // Find the octa that holds the byte
        let octa: *const Octa = self.index_mut(OctaAt(idx.0));

        // Calculate the byte's position within that octa
        let mut pos = (idx.0 % 8 / 1) as isize;
        if cfg!(target_endian = "little") {
            pos = (8 - 1) - pos;
        }

        // Calculate a pointer to the byte
        let mut byte = octa as *mut Byte;
        byte = unsafe {
            byte.offset(pos)
        };

        // Return the pointer as reference
        unsafe {
            byte.as_mut()
        }.unwrap()
    }
}

impl Index<WydeAt> for Memory {
    type Output = Wyde;
    fn index(&self, idx: WydeAt) -> &Self::Output {
        // Find the octa that holds the wyde
        let octa: *const Octa = self.index(OctaAt(idx.0));

        // Calculate the wyde's position within that octa
        let mut pos = (idx.0 % 8 / 2) as isize;
        if cfg!(target_endian = "little") {
            pos = (4 - 1) - pos;
        }

        // Calculate a pointer to the wyde
        let mut wyde = octa as *const Wyde;
        wyde = unsafe {
            wyde.offset(pos)
        };

        // Return the pointer as reference
        unsafe {
            wyde.as_ref()
        }.unwrap()
    }
}

impl IndexMut<WydeAt> for Memory {
    fn index_mut(&mut self, idx: WydeAt) -> &mut Self::Output {
        // Find the octa that holds the wyde
        let octa: *const Octa = self.index_mut(OctaAt(idx.0));

        // Calculate the wyde's position within that octa
        let mut pos = (idx.0 % 8 / 2) as isize;
        if cfg!(target_endian = "little") {
            pos = (4 - 1) - pos;
        }

        // Calculate a pointer to the wyde
        let mut wyde = octa as *mut Wyde;
        wyde = unsafe {
            wyde.offset(pos)
        };

        // Return the pointer as reference
        unsafe {
            wyde.as_mut()
        }.unwrap()
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
            pos = (2 - 1) - pos;
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
    fn index_mut(&mut self, idx: TetraAt) -> &mut Self::Output {
        // Find the octa that holds the tetra
        let octa: *mut Octa = self.index_mut(OctaAt(idx.0));

        // Calculate the tetra's position within that octa
        let mut pos = (idx.0 % 8 / 4) as isize;
        if cfg!(target_endian = "little") {
            pos = (2 - 1) - pos;
        }

        // Calculate a pointer to the tetra
        let mut tetra = octa as *mut Tetra;
        tetra = unsafe {
            tetra.offset(pos)
        };

        // Return the pointer as reference
        unsafe {
            tetra.as_mut()
        }.unwrap()
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
