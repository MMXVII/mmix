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

#[cfg(test)]
mod tests {
    use super::*;

    /// Check if any byte is accessible
    #[test]
    fn undersized() {
        let mut mem = Memory::with_capacity(25);

        //  0..25: regular bytes
        // 25..32: "alignment" bytes
        for i in 0..32 {
            mem[ByteAt(i)] = 42u8.into();
            let res: u8 = mem[ByteAt(i)].into();
            assert_eq!(res, 42u8);
        }
    }

    /// Check if the memory is not oversized
    #[test]
    #[should_panic]
    fn oversized() {
        let mut mem = Memory::with_capacity(25);
        mem[ByteAt(32)] = 42u8.into();
    }

    /// Check if the memory is initially zeroed
    #[test]
    fn zeroed() {
        let mem = Memory::with_capacity(24);

        for i in 0..24 {
            let zero: u8 = mem[ByteAt(i)].into();
            assert_eq!(zero, 0);
        }
    }

    /// Check if a byte is properly accessed
    #[test]
    fn access_byte() {
        let mut mem = Memory::with_capacity(24);

        // Write
        mem[ByteAt(8)] = 0xFFu8.into();

        // Read
        let  byte:  u8 = mem[ ByteAt(8)].into();
        assert_eq!( byte, 0xFF);

        let  wyde: u16 = mem[ WydeAt(8)].into();
        assert_eq!( wyde, 0xFF00);

        let tetra: u32 = mem[TetraAt(8)].into();
        assert_eq!(tetra, 0xFF00_0000);

        let  octa: u64 = mem[ OctaAt(8)].into();
        assert_eq!( octa, 0xFF00_0000_0000_0000);

        // Check if anything else is zero
        for i in (0..8).chain(9..24) {
            let zero: u8 = mem[ByteAt(i)].into();
            assert_eq!(zero, 0);
        }
    }

    // Check if a wyde is properly accessed
    #[test]
    fn access_wyde() {
        let mut mem = Memory::with_capacity(24);

        // Write
        mem[WydeAt(8)] = 0xFF8Fu16.into();

        // Read
        let  byte_0:  u8 = mem[ ByteAt( 8)].into();
        let  byte_1:  u8 = mem[ ByteAt( 9)].into();
        assert_eq!( byte_0, 0xFF);
        assert_eq!( byte_1, 0x8F);

        let  wyde  : u16 = mem[ WydeAt( 8)].into();
        assert_eq!( wyde  , 0xFF8F);

        let tetra  : u32 = mem[TetraAt( 8)].into();
        assert_eq!(tetra  , 0xFF8F_0000);

        let  octa  : u64 = mem[ OctaAt( 8)].into();
        assert_eq!( octa  , 0xFF8F_0000_0000_0000);

        // Check if anything else is zero
        for i in (0..8).chain(10..24) {
            let zero: u8 = mem[ByteAt(i)].into();
            assert_eq!(zero, 0);
        }
    }

    // Check if a tetra is properly accessed
    #[test]
    fn access_tetra() {
        let mut mem = Memory::with_capacity(24);

        // Write
        mem[TetraAt(8)] = 0xFF8F_8848u32.into();

        // Read
        let  byte_0:  u8 = mem[ ByteAt( 8)].into();
        let  byte_1:  u8 = mem[ ByteAt( 9)].into();
        let  byte_2:  u8 = mem[ ByteAt(10)].into();
        let  byte_3:  u8 = mem[ ByteAt(11)].into();
        assert_eq!( byte_0, 0xFF);
        assert_eq!( byte_1, 0x8F);
        assert_eq!( byte_2, 0x88);
        assert_eq!( byte_3, 0x48);

        let  wyde_0: u16 = mem[ WydeAt( 8)].into();
        let  wyde_1: u16 = mem[ WydeAt(10)].into();
        assert_eq!( wyde_0, 0xFF8F);
        assert_eq!( wyde_1, 0x8848);

        let tetra  : u32 = mem[TetraAt( 8)].into();
        assert_eq!(tetra  , 0xFF8F_8848);

        let  octa  : u64 = mem[ OctaAt( 8)].into();
        assert_eq!( octa  , 0xFF8F_8848_0000_0000);

        // Check if anything else is zero
        for i in (0..8).chain(12..24) {
            let zero: u8 = mem[ByteAt(i)].into();
            assert_eq!(zero, 0);
        }
    }

    // Check if a octa is properly accessed
    #[test]
    fn access_octa() {
        let mut mem = Memory::with_capacity(24);

        // Write
        mem[OctaAt(8)] = 0xFF8F_8848_4424_2212u64.into();

        // Read
        let  byte_0:  u8 = mem[ ByteAt( 8)].into();
        let  byte_1:  u8 = mem[ ByteAt( 9)].into();
        let  byte_2:  u8 = mem[ ByteAt(10)].into();
        let  byte_3:  u8 = mem[ ByteAt(11)].into();
        let  byte_4:  u8 = mem[ ByteAt(12)].into();
        let  byte_5:  u8 = mem[ ByteAt(13)].into();
        let  byte_6:  u8 = mem[ ByteAt(14)].into();
        let  byte_7:  u8 = mem[ ByteAt(15)].into();
        assert_eq!( byte_0, 0xFF);
        assert_eq!( byte_1, 0x8F);
        assert_eq!( byte_2, 0x88);
        assert_eq!( byte_3, 0x48);
        assert_eq!( byte_4, 0x44);
        assert_eq!( byte_5, 0x24);
        assert_eq!( byte_6, 0x22);
        assert_eq!( byte_7, 0x12);

        let  wyde_0: u16 = mem[ WydeAt( 8)].into();
        let  wyde_1: u16 = mem[ WydeAt(10)].into();
        let  wyde_2: u16 = mem[ WydeAt(12)].into();
        let  wyde_3: u16 = mem[ WydeAt(14)].into();
        assert_eq!( wyde_0, 0xFF8F);
        assert_eq!( wyde_1, 0x8848);
        assert_eq!( wyde_2, 0x4424);
        assert_eq!( wyde_3, 0x2212);

        let tetra_0: u32 = mem[TetraAt( 8)].into();
        let tetra_1: u32 = mem[TetraAt(12)].into();
        assert_eq!(tetra_0, 0xFF8F_8848);
        assert_eq!(tetra_1, 0x4424_2212);

        let  octa  : u64 = mem[ OctaAt( 8)].into();
        assert_eq!( octa  , 0xFF8F_8848_4424_2212);

        // Check if anything else is zero
        for i in (0..8).chain(16..24) {
            let zero: u8 = mem[ByteAt(i)].into();
            assert_eq!(zero, 0);
        }
    }
}
