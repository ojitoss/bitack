pub struct Writter {
    pub bytes: Vec<u8>,
    pub bytes_added: u32,
}

impl Writter {
    pub(crate) fn write_in_loop(write_bytes: &mut Vec<u8>, mut acc: u32, chunk: usize, mask: u32, bits_amount: u32) { 
        let applieds_u8 = mask.to_be_bytes();
        let current_write_bytes_chunk = &mut write_bytes[chunk..(chunk + 4)];

        for j in 0..4 {
            let apply = applieds_u8[j];
            current_write_bytes_chunk[j] |= apply;
        }
        
        acc += bits_amount;
    }

    pub fn unwrap(&self, padding: bool) -> Vec<u8> {
        let mut bytes = self.bytes.clone();
        
        if padding { return bytes };

        for _ in 0..self.bytes_added {
            bytes.pop();
        }

        bytes
    }
}