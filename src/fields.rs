use crate::{utils};
use utils::bitmask::BitMaskApplicator;

pub enum BitField {
    Next(u32),
    Skip(u32),
    LeadingZeros(u32),
    LeadingOnes(u32)
}

pub struct Resolver {
    pub applicator: BitMaskApplicator<u32>,
    pub resolver: ResolverType,
}

pub enum ResolverType {
    Base,
    LeadingOnes,
    LeadingZeros,
}

pub struct ResolverOutput {
    pub resolver: Option<Resolver>,
    pub acc: u32
}

impl BitField {
    pub fn get_bits(&self) -> u32 {
        match self {
            BitField::Next(b)
            | BitField::Skip(b)
            | BitField::LeadingOnes(b)
            | BitField::LeadingZeros(b)
            => *b
        }
    }
    pub fn resolve(&self, acc: u32) -> ResolverOutput {
        let bits = self.get_bits();

        let resolver = match self {
            BitField::Skip(..) => None,
            BitField::Next(..) => Some(ResolverType::Base),
            BitField::LeadingOnes(..) => Some(ResolverType::LeadingOnes),
            BitField::LeadingZeros(..) => Some(ResolverType::LeadingZeros),
        };

        let resolver = match resolver {
            Some(resolver_type) => Some(
                Resolver {
                    applicator: BitMaskApplicator::new(bits as usize, acc),
                    resolver: resolver_type
                }
            ),
            None => None
        };

        ResolverOutput { 
            resolver: resolver,
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
