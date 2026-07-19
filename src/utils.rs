use std::ops::{BitOr, Shl};

trait BitUint:
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

pub(crate) struct BitMaskInfo<T> {
    shift: usize,
    mask: T
}

pub(crate) fn left_bitmask_info<T>(bit_amount: usize) -> BitMaskInfo<T> where T: BitUint {
    println!("{}", bit_amount);
    let bits_minus_one = T::BITS - 1;
    let mut current_mask = T::from(0);
    for i in 0..bit_amount {
        /*
         * Select the left-most bit in relative to the 'index'.
         * Formula: 1 << ((bits - 1) - i)
         * Example: 
                        1 << (7 - 2)
                [0] [1] [2] [3] [4] [5] [6] [7]
                 1   0   1   0   1   0   0   1
                         ^ Get this bit mask.
        */
        let mask = T::from(1) << (bits_minus_one - i);
        current_mask = current_mask | mask;

        if i == (bit_amount - 1) {
            let shift = T::BITS - bit_amount;
            return BitMaskInfo { shift, mask: current_mask }
        }
    };

    BitMaskInfo { shift: 0, mask: T::from(0) }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use super::*;

    fn left_bitsmak_pattern_uints<T>(cases: Vec<(usize, usize, T)>)
    where
        T: BitUint + Debug + PartialEq
    {
        for (bits, shift, mask) in cases {
            let result = left_bitmask_info::<T>(bits);
            assert_eq!(shift, result.shift);
            assert_eq!(mask, result.mask);
        }
    }
    #[test]
    fn left_bitmask_uints() {
        left_bitsmak_pattern_uints::<u8>(vec![
            (1, 7, 0b10000000),
            (4, 4, 0b11110000),
            (8, 0, 0xff),
        ]);

        left_bitsmak_pattern_uints::<u16>(vec![
            (1, 15, 0x8000),
            (8, 8, 0xff00),
            (16, 0, 0xffff),
        ]);

        left_bitsmak_pattern_uints::<u32>(vec![
            (1, 31, 0x80000000),
            (16, 16, 0xffff0000),
            (32, 0, 0xffffffff),
        ]);
    }
}