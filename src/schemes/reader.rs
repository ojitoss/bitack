use crate::{BitScheme, fields};

pub struct Reader<'a> {
    pub bytes_added: i32,
    pub target: &'a BitScheme,
    pub bytes: Vec<u32>
}

impl Reader<'_> {
    pub fn get(&self, index: usize) -> u32 {
        let byte_index = ((index as f64) / 4.0).trunc() as usize;
        let byte = self.bytes[byte_index];

        match self.target.masks[index] {
            fields::Resolver::Base { shift, mask, .. } => (byte >> shift) & mask,
            fields::Resolver::LeadingOnes { shift, mask, .. } => ((byte & mask) << shift).leading_ones(),
            fields::Resolver::LeadingZeros { shift, mask, .. } => ((byte & mask) << shift).leading_zeros(),
        }
    }
}