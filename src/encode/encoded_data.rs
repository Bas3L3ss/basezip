use crate::huffman::CodeTable;

use super::Header;

#[derive(Debug)]
pub struct EncodedData {
    pub header: Header,
    pub data: Vec<u8>,
}

impl EncodedData {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize());

        out.extend(self.data.clone());

        out
    }

    pub fn encode_bytes(original: &[u8], code_table: &CodeTable) -> Vec<u8> {
        let mut out = Vec::new();
        let mut buffer: u8 = 0;
        let mut bits_filled: u8 = 0;

        for &byte in original {
            let (code, len) = match code_table[byte as usize] {
                Some(v) => v,
                None => { panic!("Unexpected symbol") }
            };

            // Read Huffman code from most significant bit to least significant bit.
            for i in (0..len).rev() {
                let bit = ((code >> i) & 1) as u8;

                buffer = (buffer << 1) | bit;
                bits_filled += 1;

                if bits_filled == 8 {
                    out.push(buffer);
                    buffer = 0;
                    bits_filled = 0;
                }
            }
        }

        // Flush remaining bits, padding with zeros.
        if bits_filled > 0 {
            buffer <<= 8 - bits_filled;
            out.push(buffer);
        }

        out
    }
}
