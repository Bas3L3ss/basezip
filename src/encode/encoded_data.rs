use super::metadata::Metadata;

#[derive(Debug)]
pub struct EncodedData {
    pub metadata: Metadata,
    pub data: Vec<u8>,
}

impl EncodedData {
    pub fn to_bytes(&self) -> Vec<u8> {
        // 1. Serialize metadata.
        // 2. Append metadata to output.
        // 3. Append compressed data.
        // 4. Return complete file contents.
        todo!()
    }
}
