use crate::{utils};
use utils::bitmask::BitMaskApplicator;

pub enum BitField {
    Next(u32),
    Skip(u32),
    LeadingZeros(u32),
    LeadingOnes(u32)
}

pub(crate) struct Resolver {
    pub(crate) applicator: BitMaskApplicator<u32>,
    pub(crate) resolver: ResolverType,
}

pub(crate) enum ResolverType {
    Base,
    LeadingOnes,
    LeadingZeros,
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

                let resolver = Resolver {
                    applicator: BitMaskApplicator::<u32>::from_left(bits_amount as usize, acc),
                    resolver: ResolverType::Base
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

                let resolver = Resolver {
                    applicator: BitMaskApplicator::<u32>::from_left(bits_amount as usize, acc),
                    resolver: ResolverType::LeadingOnes
                 };

                ResolverOutput { 
                    resolver: Some(resolver), 
                    acc: acc + bits_amount
                }
            },
            BitField::LeadingZeros(bits_amount) => {
                let bits_amount = *bits_amount;

                let resolver = Resolver {
                    applicator: BitMaskApplicator::<u32>::from_left(bits_amount as usize, acc),
                    resolver: ResolverType::LeadingZeros
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

    fn unwrap_resolver_info(resolver: &ResolverOutput) -> u32 {
        if let Some(resolver) = &resolver.resolver {
            return resolver.applicator.mask;
        }

        0
    }

    #[test]
    fn resolver() {
        let cases = vec![
            (BitField::Next(8).resolve(0), 0xFF << 24),
            (BitField::Next(3).resolve(0), 0xE0 << 24),
            (BitField::Next(3).resolve(16), 0xE0 << 8),
            (BitField::LeadingOnes(4).resolve(0), 0xF0 << 24),
            (BitField::LeadingZeros(4).resolve(8), 0xF0 << 16),
        ];
        
        for (resolver, expected_mask) in cases {
            let mask = unwrap_resolver_info(&resolver);
            
            assert_eq!(expected_mask, mask);
        }
    }
}
