use crate::{utils::bitmask::BitMaskApplicator};

pub enum BitField {
    Next(u32),
    Skip(u32),
    LeadingZeros(u32),
    LeadingOnes(u32)
}

pub(crate) enum Resolver {
    Base {
        shift: u32,
        mask: u32,
        bits_amount: u32
    },
    LeadingOnes {
        shift: u32,
        mask: u32,
        bits_amount: u32
    },
    LeadingZeros {
        shift: u32,
        mask: u32,
        bits_amount: u32
    },
}

pub(crate) struct ResolverOutput {
    pub(crate) resolver: Option<Resolver>,
    pub(crate) acc: u32
}

impl BitField {
    pub fn resolve(&self, acc: u32) -> ResolverOutput {
        match self {
            BitField::Next(bits_amount) => {
                let bits_amount = *bits_amount;
                let mask_info = BitMaskApplicator::<u32>::from_right(bits_amount as usize);

                let resolver = Resolver::Base {
                    shift: (mask_info.shift - (acc as usize)) as u32,
                    mask: mask_info.mask,
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
            },
            BitField::LeadingOnes(bits_amount) => {
                let bits_amount = *bits_amount;
                let mask_info = BitMaskApplicator::<u32>::from_left(bits_amount as usize);

                let resolver = Resolver::LeadingOnes { 
                    shift: acc,
                    mask: mask_info.mask >> acc,
                    bits_amount
                 };

                ResolverOutput { 
                    resolver: Some(resolver), 
                    acc: acc + bits_amount
                }
            },
            BitField::LeadingZeros(bits_amount) => {
                let bits_amount = *bits_amount;
                let mask_info = BitMaskApplicator::<u32>::from_left(bits_amount as usize);

                let resolver = Resolver::LeadingZeros { 
                    shift: acc,
                    mask: mask_info.mask >> acc,
                    bits_amount
                 };

                ResolverOutput { 
                    resolver: Some(resolver), 
                    acc: acc + bits_amount,
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn unwrap_resolver_info(resolver: &ResolverOutput) -> (u32, u32) {
        if let Some(type_resolver) = &resolver.resolver {
            if let Resolver::Base { mask, shift, .. } = type_resolver { 
                return (*mask, *shift) 
            };
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