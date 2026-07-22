pub struct Writter {
    pub bytes: Vec<u8>,
    pub bytes_added: u32,
}

impl Writter {
    pub fn unwrap(&self, padding: bool) -> Vec<u8> {
        if padding { return self.bytes.clone() };

        let mut bytes = self.bytes.clone();

        for _ in 0..self.bytes_added {
            bytes.pop();
        }

        bytes
    }
}