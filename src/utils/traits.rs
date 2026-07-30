use std::{fmt::Debug, ops::{BitOr, Shl, Shr}};

pub trait BitUint:
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

    fn leading_ones(&self) -> u32;
    fn leading_zeros(&self) -> u32;
}

impl BitUint for u8  { 
    const BITS: usize = 8; 
    const MAX: Self = !0;

    fn leading_ones(&self) -> u32 { u8::leading_ones(*self) }
    fn leading_zeros(&self) -> u32 { u8::leading_zeros(*self) }
}

impl BitUint for u16 { 
    const BITS: usize = 16;
    const MAX: Self = !0;

    fn leading_ones(&self) -> u32 { u16::leading_ones(*self) }
    fn leading_zeros(&self) -> u32 { u16::leading_zeros(*self) }
}

impl BitUint for u32 { 
    const BITS: usize = 32;
    const MAX: Self = !0;

    fn leading_ones(&self) -> u32 { u32::leading_ones(*self) }
    fn leading_zeros(&self) -> u32 { u32::leading_zeros(*self) }
}

impl BitUint for u64 { 
    const BITS: usize = 64; 
    const MAX: Self = !0;

    fn leading_ones(&self) -> u32 { u64::leading_ones(*self) }
    fn leading_zeros(&self) -> u32 { u64::leading_zeros(*self) }
}

impl BitUint for u128 {
    const BITS: usize = 128;
    const MAX: Self = !0;

    fn leading_ones(&self) -> u32 { u128::leading_ones(*self) }
    fn leading_zeros(&self) -> u32 { u128::leading_zeros(*self) } 
}
