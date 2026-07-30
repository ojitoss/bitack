use crate::{BitScheme, fields, utils};
use fields::ResolverType;
use utils::bitmask::MaskChunks;

pub struct Reader<'a> {
    pub bytes_added: i32,
    pub target: &'a BitScheme,
    pub bytes: Vec<u32>
}

impl Reader<'_> {
    pub fn get(&self, index: usize) -> u32 {
        let byte = self.bytes[0];
        let resolver = &self.target.masks[index];

        let mask = resolver.applicator.mask;
        let MaskChunks { left, right, .. } = resolver.applicator.get_chunks();

        let apply_mask = byte & mask;
        let mask_to_left = apply_mask << left;

        match resolver.resolver {
            ResolverType::Base => apply_mask >> right,
            ResolverType::LeadingOnes => mask_to_left.leading_ones(),
            ResolverType::LeadingZeros => mask_to_left.leading_zeros(),
        }
    }
}
