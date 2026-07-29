use crate::{BitScheme, fields, utils::bitmask::MaskChunks};

pub struct Reader<'a> {
    pub bytes_added: i32,
    pub target: &'a BitScheme,
    pub bytes: Vec<u32>
}

impl Reader<'_> {
    pub fn get(&self, index: usize) -> u32 {
        let byte_index = ((index as f64) / 4.0).trunc() as usize;
        let byte = self.bytes[byte_index];
        let resolver = &self.target.masks[index];
        let mask = resolver.applicator.mask;
        let MaskChunks { left, bits, right } = resolver.applicator.get_chunks();

        match resolver.resolver {
            fields::ResolverType::Base => (byte & mask) >> right,
            fields::ResolverType::LeadingOnes => ((byte & mask) << left).leading_ones(),
            fields::ResolverType::LeadingZeros => ((byte & mask) << left).leading_zeros(),
        }
    }
}
