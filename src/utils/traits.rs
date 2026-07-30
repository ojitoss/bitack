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

macro_rules! impl_bituint {
    ($($uint_type:ty),+ $(,)?) => {
        $(
            impl BitUint for $uint_type {
                const BITS: usize = <$uint_type>::BITS as usize;
                const MAX: Self = <$uint_type>::MAX;

                fn leading_ones(&self) -> u32 {
                    <$uint_type>::leading_ones(*self)
                }

                fn leading_zeros(&self) -> u32 {
                    <$uint_type>::leading_zeros(*self)
                }
            }
        )+
    };
}

impl_bituint!(u8, u16, u32, u64, u128);