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
