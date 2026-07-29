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
        let bits = match self {
            BitField::Next(b)
            | BitField::Skip(b)
            | BitField::LeadingOnes(b)
            | BitField::LeadingZeros(b)
            => *b
        };

        let resolver = match self {
            BitField::Next(..) => ResolverType::Base,
            BitField::LeadingOnes(..) => ResolverType::LeadingOnes,
            BitField::LeadingZeros(..) => ResolverType::LeadingZeros,
            _ => ResolverType::Base
        };

        let resolver = {
            if let BitField::Skip(..) = self { None }
            else { Some(Resolver {
                    applicator: BitMaskApplicator::new(bits as usize, acc),
                    resolver: resolver
                })
            }
        };

        ResolverOutput { 
            resolver,
            acc: acc + bits
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
