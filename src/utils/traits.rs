use std::{fmt::Debug, ops::{BitOr, Shl, Shr}};

pub(crate) trait BitUint:
    Copy
    + From<u8>
    + BitOr<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Debug
    + PartialEq
{
    const BITS: usize;
    const MAX: Self;
}

impl BitUint for u8  { 
    const BITS: usize = 8; 
    const MAX: Self = !0;
}

impl BitUint for u16 { 
    const BITS: usize = 16;
    const MAX: Self = !0;
}

impl BitUint for u32 { 
    const BITS: usize = 32;
    const MAX: Self = !0;
}

impl BitUint for u64 { 
    const BITS: usize = 64; 
    const MAX: Self = !0;
}

impl BitUint for u128 {
    const BITS: usize = 128;
    const MAX: Self = !0;
}
