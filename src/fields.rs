use crate::{fields, utils::bitmask::left_bitmask_info};

pub enum BitField {
    Next(u32),
    Skip(u32)
}

pub(crate) enum Resolvers {
    Base {
        shift: u32,
        mask: u32,
        bits_amount: u32
    }
}

pub(crate) struct ResolverOutput {
    resolver: Option< Resolvers>,
    acc: u32
}

impl BitField {
    pub fn resolve(&self, acc: u32) -> ResolverOutput {
        match self {
            BitField::Next(bits_amount) => {
                let bits_amount = *bits_amount;
                let mask_info = left_bitmask_info::<u32>(bits_amount as usize);

                let resolver = Resolvers::Base {
                    shift: (mask_info.shift - (acc as usize)) as u32,
                    mask: mask_info.mask >> mask_info.shift,
                    bits_amount
                };

                ResolverOutput {
                    resolver: Some(resolver),
                    acc: acc + bits_amount
                }
            },
            BitField::Skip(bits_amount) => {
                ResolverOutput {
                    resolver: None,
                    acc: acc + bits_amount
                }
            }
        }
    }
}

pub(crate) fn resvoler(fields: Vec<BitField>) -> Vec<Resolvers> {
    let mut masks: Vec<Resolvers> = vec![];
    let mut resolver = ResolverOutput {
        resolver: None,
        acc: 0
    };

    for field in &fields {
        resolver = field.resolve(resolver.acc);
        
        if let Some(resolver) = resolver.resolver {
            masks.push(resolver);
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
            if let Resolvers::Base { shift, mask, .. } = resolver {
                assert_eq!(expected_shift, shift);
                assert_eq!(expected_mask, mask);
            }
        }
    }
}
