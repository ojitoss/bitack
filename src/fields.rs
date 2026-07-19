use crate::utils::{left_bitmask_info};

pub enum BitField {
    Next(u32),
    Skip(u32)
}

pub(crate) enum Resolvers {
    Base {
        shift: u32,
        mask: u32
    }
}

pub(crate) fn resvoler(bits: Vec<BitField>) -> Vec<Resolvers> {
    let mut acc: u32 = 0;
    let mut masks: Vec<Resolvers> = vec![];

    for bit in &bits {
        match bit {
            BitField::Next(bit_amount) => {
                let mask_info = left_bitmask_info::<u32>(*bit_amount as usize);

                masks.push(Resolvers::Base {
                    shift: (mask_info.shift - (acc as usize)) as u32,
                    mask: mask_info.mask >> mask_info.shift
                });
                
                acc += bit_amount;
            },
            BitField::Skip(bit_amount) => {
                acc += bit_amount;
            }
        }
    }

    masks
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resolver() {
        let layout = vec![
            BitField::Next(2),
            BitField::Next(6),
            BitField::Skip(4),
            BitField::Next(4),
        ];

        let resolvers = resvoler(layout);
        assert_eq!(3, resolvers.len());

        let cases: Vec<(&u32, &u32)> = vec![
            (&0b11, &30),
            (&0b00_111111, &24),
            (&0b00000000_0000_1111, &16)
        ];

        for i in 0..cases.len() {
            let resolver = &resolvers[i];
            let (expected_mask, expected_shift) = cases[i];

            #[allow(irrefutable_let_patterns)]
            if let Resolvers::Base { shift, mask } = resolver {
                println!("df");
                assert_eq!(expected_shift, shift);
                assert_eq!(expected_mask, mask);
            }
        }
    }
}