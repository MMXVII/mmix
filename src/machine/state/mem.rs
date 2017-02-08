use machine::state::types::*;

use std::mem::transmute;
use std::ops::{Index, IndexMut};

pub struct ByteAt(pub u64);
pub struct WydeAt(pub u64);
pub struct TetraAt(pub u64);
pub struct OctaAt(pub u64);

pub struct Memory(Vec<u8>);

// 2^30
// TODO let the user choose the size
const MAX_ADDRESS : usize = 0x40000000;

static ZERO_BYTE : Byte = Byte(0);
static ZERO_WYDE: Wyde = Wyde(0);

impl Memory {
    pub fn new() -> Self {
        Memory(Vec::new())
    }
}

impl Index<ByteAt> for Memory {
    type Output = Byte;
    fn index(&self, index: ByteAt) -> &Self::Output {
        let i = index.0 as usize;
        assert!(i < MAX_ADDRESS);
        println!("{} >= {}?", i, self.0.len());
        if i >= self.0.len() {
            &ZERO_BYTE
        } else {
            let raw_ref = self.0.get(i).unwrap();
            unsafe {
                transmute::<&u8, &Byte>(raw_ref)
            }
        }
    }
}

impl IndexMut<ByteAt> for Memory {
    fn index_mut(&mut self, index: ByteAt) -> &mut Self::Output {
        let i = index.0 as usize;
        assert!(i < MAX_ADDRESS);
        if i >= self.0.len() {
            // make the new size a multiple of 8, so that no problems arise for
            // Wydes, Tetras and Octas
            let new_size = (i | 111) + 1;
            self.0.resize(new_size, 0);
        }
        let raw_ref = self.0.get_mut(i).unwrap();
        unsafe {
            transmute::<&mut u8, &mut Byte>(raw_ref)
        }
    }
}

impl Index<WydeAt> for Memory {
    type Output = Wyde;
    fn index(&self, index: WydeAt) -> &Self::Output {

        let lo = (index.0 | 1) as usize;
        let hi = lo - 1;

        assert!(lo < MAX_ADDRESS);

        if lo >= self.0.len() {
            &ZERO_WYDE
        } else {
            println!("vector length: {}", self.0.len());
            println!("requested address: {}", index.0);
            println!("high byte at index {}: {}", hi, self.0[hi]);
            println!("low byte at index {}: {}", lo, self.0[lo]);
            unsafe {
                transmute::<&(u8, u8), &Wyde>(&(self.0[hi], self.0[lo]))

            }
        }



    }
}

impl IndexMut<WydeAt> for Memory {
    fn index_mut(&mut self, index: WydeAt) -> &mut Self::Output {

        let lo = (index.0 | 1) as usize;
        let hi = lo - 1;

        assert!(lo < MAX_ADDRESS);

        if lo >= self.0.len() {
            let new_size = (lo | 111) + 1;
            self.0.resize(new_size, 0);
        }
        unsafe {
            transmute::<&mut (u8, u8), &mut Wyde>(&mut (self.0[hi], self.0[lo]))
        }
    }
}

impl Index<TetraAt> for Memory {
    type Output = Tetra;
    fn index(&self, _: TetraAt) -> &Self::Output {
        unimplemented!();
    }
}

impl IndexMut<TetraAt> for Memory {
    fn index_mut(&mut self, _: TetraAt) -> &mut Self::Output {
        unimplemented!();
    }
}

impl Index<OctaAt> for Memory {
    type Output = Octa;
    fn index(&self, _: OctaAt) -> &Self::Output {
        unimplemented!();
    }
}

impl IndexMut<OctaAt> for Memory {
    fn index_mut(&mut self, _: OctaAt) -> &mut Self::Output {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_index() {
        let mut mem = Memory::new();
        assert_eq!(mem[ByteAt(0)].0, 0u8);
        assert_eq!(mem[ByteAt(10)].0, 0u8);
        mem[ByteAt(100)] = 10u8.into();
        assert_eq!(mem[ByteAt(100)], 10u8.into());
    }

    #[test]
    fn wyde_index() {
        let mut mem = Memory::new();
        assert_eq!(mem[WydeAt(12345)].0, 0u16);
        mem[WydeAt(123)] = 42u16.into();
        assert_eq!(mem[WydeAt(123)], 42u16.into());

    }


}
