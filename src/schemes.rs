mod reader;
mod writter;
use reader::Reader;
use writter::Writter;
use crate::fields;

pub struct BitScheme {
    masks: Vec<fields::Resolvers>
}

impl BitScheme {
    pub fn new(bits: Vec<fields::BitField>) -> Self {
        BitScheme { 
            masks: fields::resvoler(bits) 
        }
    }

    pub fn read(&self, origin_bytes: Vec<u8>) -> Reader {
        let mut bytes = origin_bytes.clone();
        
        // Add padding at the bytes to can be compressed exactly.
        let max = (4 - (bytes.len() % 4)) % 4;
        for _ in 0..max {
            bytes.push(0);
        }

        // Compress 'Vec<u8>' to 'Vec<u32>'.
        let bytes = bytes
            .chunks_exact(4)
            .map(| chunk | u32::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        Reader {
            bytes_added: max as i32,
            target: self,
            bytes
        }
    }

    pub fn write(&self, bytes: Vec<u32>) -> Writter {
        let mut chunk = 0;
        let mut acc = 0;
        let mut write_bytes: Vec<u8> = vec![0, 0, 0, 0];

        for i in 0..bytes.len() {
            let byte = bytes[i];
            let resolver = &self.masks[i];

            match resolver {
                fields::Resolvers::Base { shift, bits_amount, .. } => {
                    let mask = byte << shift;
                    let applieds_u8 = mask.to_be_bytes();
                    let current_write_bytes_chunk = &mut write_bytes[chunk..(chunk + 4)];

                    for j in 0..4 {
                        let apply = applieds_u8[j];
                        current_write_bytes_chunk[j] |= apply;
                    }

                    acc += bits_amount;
                }
            }

            if acc >= 32 {
                acc -= 32;
                chunk += 1;
                for _ in 0..4 { write_bytes.push(0) }
            }
        }

        let bytes_added = if (acc % 8) == 0 { acc / 8 } else { (((acc as f64) / 8.0).trunc() as u32) + 1 };

        Writter {
            bytes: write_bytes,
            bytes_added
        }
    }
}