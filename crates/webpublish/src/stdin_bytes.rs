use std::io::{self, Read};

#[derive(Debug, Clone)]
pub struct StdinBytes {
    bytes: Vec<u8>,
}

impl StdinBytes {
    pub fn from_stdin() -> io::Result<Self> {
        let mut bytes = Vec::new();
        io::stdin().read_to_end(&mut bytes)?;
        Ok(Self { bytes })
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}
