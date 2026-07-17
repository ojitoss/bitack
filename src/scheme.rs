use crate::fields; 
pub struct BitScheme {
    masks: Vec<fields::Resolvers>
}

pub struct SchemeConsumer<'a> {
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

    pub fn consume(&self, origin_bytes: Vec<u8>) -> SchemeConsumer {
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

        SchemeConsumer {
            bytes_added: max as i32,
            target: self,
            bytes
        }
    }
}

impl SchemeConsumer<'_> {
    pub fn get(&self, index: usize) -> u32 {
        match self.target.masks[index] {
            fields::Resolvers::Base { shift, mask } => {
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
    fn consumer_getter() {
        let scheme = BitScheme::new(vec![
            fields::BitField::Next(2),
            fields::BitField::Next(6),
            fields::BitField::Skip(4),
            fields::BitField::Next(4),
        ]);
        let scheme = scheme.consume(vec![
            0b10_000010,
            0b1101_1111,
            0b0,
            0b0
        ]);
        
        assert_eq!(2, scheme.get(0));
        assert_eq!(2, scheme.get(1));
        assert_eq!(15, scheme.get(2));
    }

    #[test]
    fn padding_len() {
        let scheme = BitScheme::new(vec![
            fields::BitField::Next(1)
        ]);

        let mut zeros: Vec<u8> = vec![];

        for i in 1..=5 {
            zeros.push(0);
            let consume = scheme.consume(zeros.clone());
            if i == 1 { assert_eq!(3, consume.bytes_added) }
            if i == 2 { assert_eq!(2, consume.bytes_added) }
            if i == 3 { assert_eq!(1, consume.bytes_added) }
            if i == 4 { assert_eq!(0, consume.bytes_added) }
            if i == 5 { assert_eq!(3, consume.bytes_added) }
        }
    }
}