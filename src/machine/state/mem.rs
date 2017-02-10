use machine::state::types::*;

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ByteAt(pub u64);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WydeAt(pub u64);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TetraAt(pub u64);

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Round the number of bytes up to next full Octa
        let mut mem = Memory::with_capacity(4); // 1 Octa of memory

        mem[OctaAt(0)] = 1u64.into();

        let fu: u64 = mem[OctaAt(0)].into();

        assert_eq!(fu, 1u64);

        let lo: u32 = mem[TetraAt(0)].into();
        let hi: u32 = mem[TetraAt(4)].into();

        assert_eq!(lo, 0);
        assert_eq!(hi, 1);

        for i in 0 .. 8 {
            mem[ByteAt(i)] = ((i + 1) as u8).into();
        }

        assert_eq!(mem[OctaAt(7)], 0x0102030405060708u64.into());

        assert_eq!(mem[TetraAt(3)], 0x01020304u32.into());
        assert_eq!(mem[TetraAt(4)], 0x05060708u32.into());

        assert_eq!(mem[WydeAt(0)], 0x0102u16.into());
        assert_eq!(mem[WydeAt(2)], 0x0304u16.into());
        assert_eq!(mem[WydeAt(4)], 0x0506u16.into());
        assert_eq!(mem[WydeAt(6)], 0x0708u16.into());

        // Test that bytes are not changed by changing bytes near them
        mem[TetraAt(0)] = 0u32.into();
        mem[WydeAt(7)] = 0u16.into();
        mem[ByteAt(4)] = 0u8.into();

        assert_eq!(mem[OctaAt(0)], 0x60000u64.into());

    }

    #[test]
    fn equivalent_indices() {

        let mut mem = Memory::with_capacity(15);

        for i in 0 .. 16 {
            mem[ByteAt(i)] = (i as u8).into();
        }

        for i in 1 .. 8 {
            assert_eq!(mem[OctaAt(0)], mem[OctaAt(i)]);
            assert_eq!(mem[OctaAt(8)], mem[OctaAt(8 + i)]);
        }

        for i in 1 .. 4 {
            assert_eq!(mem[TetraAt(0)], mem[TetraAt(i)]);
            assert_eq!(mem[TetraAt(4)], mem[TetraAt(4 + i)]);
            assert_eq!(mem[TetraAt(8)], mem[TetraAt(8 + i)]);
            assert_eq!(mem[TetraAt(12)], mem[TetraAt(12 + i)]);
        }

        for i in 0 .. 8 {
            assert_eq!(mem[WydeAt(2 * i)], mem[WydeAt(2 * i + 1)]);
        }



    }

}
