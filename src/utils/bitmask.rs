use crate::utils::traits::{BitUint};

pub(crate) struct BitMaskInfo<T> {
    pub(crate) shift: usize,
    pub(crate) mask: T
}

#[derive(Clone, Copy)]
pub(crate) enum BitShiftDirection {
    Left,
    Right
}

pub(crate) fn bitmask_info<T>(bits_amount: usize, shift_type: BitShiftDirection) -> BitMaskInfo<T> where T: BitUint {
    let shift = T::BITS - bits_amount;

    let mask = match shift_type {
        BitShiftDirection::Left => T::MAX << shift,
        BitShiftDirection::Right => T::MAX >> shift
    };

    BitMaskInfo { shift, mask }
}

#[cfg(test)]
mod test {
    use super::*;

    fn bitsmak_pattern_uints<T>(cases: Vec<(usize, usize, T)>, shift_type: BitShiftDirection)  where T: BitUint {
        for (bits, shift, mask) in cases {
            let result = bitmask_info::<T>(bits, shift_type);

            assert_eq!(shift, result.shift);
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
