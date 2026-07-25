use crate::utils::traits::{BitUint};

pub(crate) struct BitMaskInfo<T> {
    pub(crate) shift: usize,
    pub(crate) mask: T
}

pub(crate) fn left_bitmask_info<T>(bits_amount: usize) -> BitMaskInfo<T> where T: BitUint {
    let shift = T::BITS - bits_amount;
    let mask = T::MAX << shift;

    BitMaskInfo { shift, mask }
}

#[cfg(test)]
mod test {
    use super::*;

    fn left_bitsmak_pattern_uints<T>(cases: Vec<(usize, usize, T)>)  where T: BitUint {
        for (bits, shift, mask) in cases {
            let result = left_bitmask_info::<T>(bits);

            assert_eq!(shift, result.shift);
            assert_eq!(mask, result.mask);
        }
    }

    fn left_bitmask_tested_pattern<T>() where T: BitUint {
        let minus_one = T::BITS - 1;
        let half =  T::BITS / 2;

        left_bitsmak_pattern_uints(vec![
            (1, minus_one,T::from(1) << minus_one),
            (half, half, T::MAX << half),
            (T::BITS, 0, T::MAX)
        ]);
    }

    #[test]
    fn left_bitmask_uints() {
        left_bitmask_tested_pattern::<u8>();
        left_bitmask_tested_pattern::<u16>();
        left_bitmask_tested_pattern::<u32>();
        left_bitmask_tested_pattern::<u64>();
        left_bitmask_tested_pattern::<u128>();   
    }
}
