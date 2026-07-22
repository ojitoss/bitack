pub struct Writter {
    pub bytes: Vec<u8>,
    pub bytes_added: u32,
}

impl Writter {
    pub fn unwrap(&self, padding: bool) -> Vec<u8> {
        let mut bytes = self.bytes.clone();
        
        if padding { return bytes };

        for _ in 0..self.bytes_added {
            bytes.pop();
        }

        bytes
    }
}