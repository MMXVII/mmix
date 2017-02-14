use machine::state::types::Octa;

use std::mem::transmute;
use std::ops::{Index, IndexMut};


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum R {
    A = 21,  B =  0,  C =  8,  D =  1,  E =  2,  F = 22,  G = 19,  H =  3,
    I = 12,  J =  4,  K = 15,  L = 20,  M =  5,  N =  9,  O = 10,  P = 23,
    Q = 16,  R =  6,  S = 11,  T = 13,  U = 17,  V = 18,  W = 24,  X = 25,
    Y = 26,  Z = 27, BB =  7, TT = 14, WW = 28, XX = 29, YY = 30, ZZ = 31,
}

impl From<u8> for R {
    fn from(from: u8) -> Self {
        assert!(from < 32);
        unsafe {
            transmute::<u8, R>(from)
        }
    }
}

impl Into<u8> for R {
    fn into(self) -> u8 {
        self as u8
    }
}


pub struct SRegisters {
    buf: Vec<Octa>,
}

impl SRegisters {
    pub fn new() -> Self {
        SRegisters {
            buf: vec![0u64.into(); 32],
        }
    }
}

impl Index<R> for SRegisters {
    type Output = Octa;
    fn index(&self, index: R) -> &Self::Output {
        self.buf.index(index as usize)
    }
}

impl IndexMut<R> for SRegisters {
    fn index_mut(&mut self, index: R) -> &mut Self::Output {
        self.buf.index_mut(index as usize)
    }
}
