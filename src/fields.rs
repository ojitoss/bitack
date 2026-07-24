use crate::{utils::bitmask::left_bitmask_info};

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
    pub(crate) resolver: Option< Resolvers>,
    pub(crate) acc: u32
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

#[cfg(test)]
mod test {
    use super::*;

    fn unwrap_resolver_info(resolver: &ResolverOutput) -> (u32, u32) {
        if let Some(y) = &resolver.resolver {
            #[allow(irrefutable_let_patterns)]
            if let Resolvers::Base { mask, shift, .. } = y { return (*mask, *shift) };
        }

        (0, 0)
    }

    #[test]
    fn resolver() {
        let cases = vec![
            (BitField::Next(8).resolve(0), 0xFF, 24),
            (BitField::Next(3).resolve(0), 0b111, 29),
            (BitField::Next(3).resolve(5), 0b111, 24),
        ];
        
        for (resolver, expected_mask, expected_shift) in cases {
            let (mask, shift) = unwrap_resolver_info(&resolver);
            assert_eq!(expected_mask, mask);
            assert_eq!(expected_shift, shift);
        }
    }
}