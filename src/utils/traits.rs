use std::ops::{BitOr, Shl};

pub(crate) trait BitUint:
    Copy
    + From<u8>
    + BitOr<Output = Self>
    + Shl<usize, Output = Self>
{
    const BITS: usize;
}

impl BitUint for u8  { const BITS: usize = 8; }
impl BitUint for u16 { const BITS: usize = 16; }
impl BitUint for u32 { const BITS: usize = 32; }
impl BitUint for u64 { const BITS: usize = 64; }
impl BitUint for u128 { const BITS: usize = 128; }
