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

impl<T: BitUint> BitMaskApplicator<T> {
    pub fn new(bits_amount: usize, acc: u32) -> Self {
        let bits_len_diff = T::BITS - bits_amount;

        let mask = {
            let left_most_mask =  T::MAX << bits_len_diff;

            left_most_mask >> (acc as usize)
        };

        let bits_len_diff = bits_len_diff as u32;

        Self { bits_len_diff, mask }
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

    fn bitsmak_generator_pattern_uints<T>(cases: Vec<(usize, usize, T)>)  where T: BitUint {
        for (bits, bits_len_diff, mask) in cases {
            let result = BitMaskApplicator::<T>::new(bits, 0);

            assert_eq!(bits_len_diff, result.bits_len_diff as usize);
            assert_eq!(mask, result.mask);
        }
    }

    fn bitmask_generator_tested_pattern<T>() where T: BitUint {
        let minus_one = T::BITS - 1;
        let half =  T::BITS / 2;

        let cases = vec![
            (1, minus_one, T::from(1) << minus_one),
            (half, half, T::MAX << half),
            (T::BITS, 0, T::MAX)
        ];

        bitsmak_generator_pattern_uints(cases);
    }

    #[test]
    fn bitmask_generator_uints() {
        bitmask_generator_tested_pattern::<u8>();
        bitmask_generator_tested_pattern::<u16>();
        bitmask_generator_tested_pattern::<u32>();
        bitmask_generator_tested_pattern::<u64>();
        bitmask_generator_tested_pattern::<u128>();   
    }
}
