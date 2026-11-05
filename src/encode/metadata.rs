use crate::huffman::CodeTable;

#[derive(Debug)]
pub struct Metadata {
    pub original_size: u64,
    pub code_table: CodeTable,
}

impl Metadata {
    pub fn new(original_size: u64, code_table: CodeTable) -> Self {
        Self { original_size, code_table }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(8 + 256 * 5);

        // Original file size
        out.extend_from_slice(&self.original_size.to_le_bytes());

        // Huffman code table
        for entry in &self.code_table {
            match entry {
                Some((code, len)) => {
                    out.push(*len);
                    out.extend_from_slice(&code.to_le_bytes());
                }
                None => {
                    out.push(0);
                    out.extend_from_slice(&(0u32).to_le_bytes());
                }
            }
        }

        out
    }
    pub fn deserialize(bytes: &[u8]) -> Self {
        todo!()
    }
}
