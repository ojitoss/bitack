use crate::utils::traits::{BitUint};

pub(crate) struct BitMaskInfo<T> {
    pub(crate) shift: usize,
    pub(crate) mask: T
}

pub(crate) fn left_bitmask_info<T>(bits_amount: usize) -> BitMaskInfo<T> where T: BitUint {
    let bits_minus_one = T::BITS - 1;
    let mut current_mask = T::from(0);

    for i in 0..bits_amount {
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

        if i == (bits_amount - 1) {
            let shift = T::BITS - bits_amount;
            return BitMaskInfo { shift, mask: current_mask }
        }
    }

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

    fn left_bitmask_tested_pattern<T>()
    where 
        T: BitUint + Debug + PartialEq
    {
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
