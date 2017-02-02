use machine::state::types::*;

use std::ops::{Index, IndexMut};

pub struct ByteAt(pub u64);
pub struct WydeAt(pub u64);
pub struct TetraAt(pub u64);
pub struct OctaAt(pub u64);

pub struct Memory(());

impl Memory {
    pub fn new() -> Self {
        unimplemented!();
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
