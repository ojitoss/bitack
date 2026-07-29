use crate::utils::traits::{BitUint};

pub(crate) struct MaskChunks {
    pub(crate) left: u32,
    pub(crate) bits: u32,
    pub(crate) right: u32
}

pub(crate) struct BitMaskApplicator<T: BitUint> {
    pub(crate) bits_len_diff: u32,
    pub(crate) mask: T,
}

#[derive(Clone, Copy)]
pub(crate) enum BitShiftDirection {
    Left,
    Right
}

impl<T: BitUint> BitMaskApplicator<T> {
    fn new(bits_amount: usize, shift_type: BitShiftDirection, acc: u32) -> Self {
        let bits_len_diff = T::BITS - bits_amount;

        let mask = match shift_type {
            BitShiftDirection::Left => {
                let left_most_mask =  T::MAX << bits_len_diff;

                left_most_mask >> (acc as usize)
            }
            BitShiftDirection::Right => {
                let right_most_mask = T::MAX >> bits_len_diff;

                right_most_mask << (acc as usize)
            }
        };

        let bits_len_diff = bits_len_diff as u32;

        Self { bits_len_diff, mask }
    }

    pub fn from_left(bits_amount: usize, acc: u32) -> Self {
        Self::new(bits_amount, BitShiftDirection::Left, acc)
    }

    pub fn from_right(bits_amount: usize, acc: u32) -> Self {
        Self::new(bits_amount, BitShiftDirection::Right, acc)
    }

    pub fn get_chunks(&self) -> MaskChunks {
        let left = self.mask.leading_zeros();
        let bits = (self.mask << (left as usize)).leading_ones();
        let right = 32 - (left + bits);

        MaskChunks { left, bits, right }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn bitsmak_pattern_uints<T>(cases: Vec<(usize, usize, T)>, shift_type: BitShiftDirection)  where T: BitUint {
        for (bits, bits_len_diff, mask) in cases {
            let result = BitMaskApplicator::<T>::new(bits, shift_type, 0);

            assert_eq!(bits_len_diff, result.bits_len_diff as usize);
            assert_eq!(mask, result.mask);
        }
    }

    fn bitmask_tested_pattern<T>(shift_type: BitShiftDirection) where T: BitUint {
        let minus_one = T::BITS - 1;
        let half =  T::BITS / 2;

        let mut cases = vec![
            (1, minus_one, T::MAX),
            (half, half, T::MAX),
            (T::BITS, 0, T::MAX)
        ];

        match shift_type {
            BitShiftDirection::Left => {
                cases[0].2 = T::from(1) << minus_one;
                cases[1].2 = T::MAX << half;
            },
            BitShiftDirection::Right => {
                cases[0].2 = T::MAX >> minus_one;
                cases[1].2 = T::MAX >> half;
            }
        };

        bitsmak_pattern_uints(cases, shift_type);
    }

    #[test]
    fn left_bitmask_uints() {
        bitmask_tested_pattern::<u8>(BitShiftDirection::Left);
        bitmask_tested_pattern::<u16>(BitShiftDirection::Left);
        bitmask_tested_pattern::<u32>(BitShiftDirection::Left);
        bitmask_tested_pattern::<u64>(BitShiftDirection::Left);
        bitmask_tested_pattern::<u128>(BitShiftDirection::Left);   
    }

    #[test]
    fn right_bitmask_uints() {
        bitmask_tested_pattern::<u8>(BitShiftDirection::Right);
        bitmask_tested_pattern::<u16>(BitShiftDirection::Right);
        bitmask_tested_pattern::<u32>(BitShiftDirection::Right);
        bitmask_tested_pattern::<u64>(BitShiftDirection::Right);
        bitmask_tested_pattern::<u128>(BitShiftDirection::Right);   
    }
}
