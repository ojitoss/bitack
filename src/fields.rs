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
    let mut acc = 0;
    let mut masks: Vec<Resolvers> = vec![];
    let mut current_mask= 0;

    for bit in &bits {
        match bit {
            BitField::Next(bit_amount) => {
                for i in 0..*bit_amount {
                    /*
                     * Select the left-most bit in relative to the 'index'.
                     * Formula: 1 << ((bits - 1) - i)
                     * Example: 
                                    1 << (7 - 2)
                            [0] [1] [2] [3] [4] [5] [6] [7]
                             1   0   1   0   1   0   0   1
                                     ^ Get this bit mask.
                    */
                    let mask: u32 = 1 << (31 - i);
                    current_mask |= mask;

                    if i == (bit_amount - 1) {
                        let shift = 32 - bit_amount;
                        masks.push(Resolvers::Base {
                            shift: shift - acc,
                            mask: current_mask >> shift
                        });
                        current_mask = 0;
                        acc += bit_amount;
                    }
                }
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