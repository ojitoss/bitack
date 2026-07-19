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
                let some = left_bitmask_info::<u32>(*bit_amount as usize);

                masks.push(Resolvers::Base {
                    shift: (some.shift - (acc as usize)) as u32,
                    mask: some.mask >> some.shift
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
        for i in 0..3 {
            let resolver = &resolvers[i];
            match resolver {
                Resolvers::Base { shift, mask } => {
                    if i == 0 {
                        assert_eq!(&0b11, mask);
                        assert_eq!(&30, shift);
                    }

                    if i == 1 {
                        assert_eq!(&0b00_111111, mask);
                        assert_eq!(&24, shift);
                    }

                    if i == 2 {
                        assert_eq!(&0b00000000_0000_1111, mask);
                        assert_eq!(&16, shift);
                    }
                }
            }
        }
    }
}