use crate::fields; 
pub struct BitScheme {
    masks: Vec<fields::Resolvers>
}

pub struct SchemeConsumer {
    target: BitScheme,
    bytes: Vec<u32>
}

impl BitScheme {
    pub fn new(bits: Vec<fields::BitField>) -> Self {
        BitScheme { 
            masks: fields::resvoler(bits) 
        }
    }

    pub fn consume(self, origin_bytes: Vec<u8>) -> SchemeConsumer {
        let mut bytes = origin_bytes.clone();
        
        // Add padding at the bytes to can be compressed exactly.
        let max = (4 - bytes.len().min(4)) % 4;
        for _ in 0..max {
            bytes.push(0);
        }

        // Compress 'Vec<u8>' to 'Vec<u32>'.
        let bytes = bytes
            .chunks_exact(4)
            .map(| chunk | u32::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        SchemeConsumer {
            target: self,
            bytes
        }
    }
}

impl SchemeConsumer {
    pub fn get(self, index: usize) -> u32 {
        match self.target.masks[index] {
            fields::Resolvers::Base { shift, mask } => {
                let byte_index = ((index as f64) / 4.0).trunc() as usize;
                let shifted = self.bytes[byte_index] >> shift;    
                return shifted & mask;
            }   
        }
    }
}
