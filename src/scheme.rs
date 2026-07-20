use crate::fields; 
pub struct BitScheme {
    masks: Vec<fields::Resolvers>
}

pub struct SchemeReader<'a> {
    bytes_added: i32,
    target: &'a BitScheme,
    bytes: Vec<u32>
}

impl BitScheme {
    pub fn new(bits: Vec<fields::BitField>) -> Self {
        BitScheme { 
            masks: fields::resvoler(bits) 
        }
    }

    pub fn read(&self, origin_bytes: Vec<u8>) -> SchemeReader {
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

        SchemeReader {
            bytes_added: max as i32,
            target: self,
            bytes
        }
    }

    pub fn write(&self, bytes: Vec<u32>) -> Vec<u8> {
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
        
        write_bytes
    }
}

impl SchemeReader<'_> {
    pub fn get(&self, index: usize) -> u32 {
        match self.target.masks[index] {
            fields::Resolvers::Base { shift, mask, .. } => {
                let byte_index = ((index as f64) / 4.0).trunc() as usize;
                let shifted = self.bytes[byte_index] >> shift;    
                return shifted & mask;
            }   
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reader_and_writter() {
        let scheme = BitScheme::new(vec![
            fields::BitField::Next(2),
            fields::BitField::Next(6),
            fields::BitField::Skip(4),
            fields::BitField::Next(4),
        ]);

        let origin_bytes = vec![
            0b10_000010,
            0b0000_1111,
            0b0,
            0b0
        ];

        let cases = vec![
            2,
            2,
            15
        ];

        let to_read_bytes = scheme.read(origin_bytes.clone());
        let writted_bytes = scheme.write(cases.clone());

        for i in 0..cases.len() {
            let case = cases[i];
            assert_eq!(case, to_read_bytes.get(i));
        }
        
        assert_eq!(writted_bytes, origin_bytes);
    }

    #[test]
    fn padding_len() {
        let scheme = BitScheme::new(vec![
            fields::BitField::Next(1)
        ]);

        let mut zeros: Vec<u8> = vec![];

        let cases = vec![
            3, // len = 1
            2, // len = 2
            1, // len = 3
            0, // len = 4
            3  // len = 5
        ];

        for i in 0..5 {
            zeros.push(0);
            let consume = scheme.read(zeros.clone());
            let case = cases[i];

            assert_eq!(case, consume.bytes_added);
        }
    }
}