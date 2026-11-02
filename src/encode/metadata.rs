#[derive(Debug)]
pub struct Metadata {
    pub original_size: u64,
    pub code_lengths: [u8; 256],
}

impl Metadata {
    pub fn new(original_size: u64, code_lengths: [u8; 256]) -> Self {
        todo!()
    }

    pub fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    pub fn deserialize(bytes: &[u8]) -> Self {
        todo!()
    }
}
