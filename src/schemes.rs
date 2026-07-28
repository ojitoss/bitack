mod reader;
mod writter;

use reader::Reader;
use writter::Writter;

use crate::fields;

pub struct BitScheme {
    masks: Vec<fields::Resolver>
}

impl BitScheme {
    pub fn new(fields: Vec<fields::BitField>) -> Self {
        let mut masks: Vec<fields::Resolver> = vec![];
        let mut resolver = fields::ResolverOutput {
            resolver: None,
            acc: 0
        };

        for field in &fields {
            resolver = field.resolve(resolver.acc);
            
            if let Some(resolver) = resolver.resolver {
                masks.push(resolver);
            }
        }

        Self { masks }
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
                fields::Resolver::Base { shift, bits_amount, .. } => {
                    let mask = byte << shift;

                    Writter::write_in_loop(&mut write_bytes, acc, chunk, mask, *bits_amount);
                },
                fields::Resolver::LeadingOnes { mask, bits_amount, .. } => {
                    let left = mask.leading_zeros();
                    let mut mask = *mask;

                    for i in ((left + byte)..(bits_amount + left)).rev() {
                        let pos: u32 = 1 << (31 - i);
                        mask &= !pos;
                    }

                    Writter::write_in_loop(&mut write_bytes, acc, chunk, mask, *bits_amount);
                },
                fields::Resolver::LeadingZeros { bits_amount, shift, .. } => {
                    let mask = 0 | (1 << (31 - (shift + byte)));

                    Writter::write_in_loop(&mut write_bytes, acc, chunk, mask, *bits_amount);
                }, 
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