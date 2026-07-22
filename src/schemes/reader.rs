use crate::{BitScheme, fields};

pub struct Reader<'a> {
    pub bytes_added: i32,
    pub target: &'a BitScheme,
    pub bytes: Vec<u32>
}

impl Reader<'_> {
    pub fn get(&self, index: usize) -> u32 {
        match self.target.masks[index] {
            fields::Resolvers::Base { shift, mask, .. } => {
                let byte_index = ((index as f64) / 4.0).trunc() as usize;
                let shifted = self.bytes[byte_index] >> shift;
                
                shifted & mask
            }   
        }
    }
}