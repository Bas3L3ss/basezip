use crate::{ encode::Header, huffman::Node };
use super::EncodedData;

pub struct Encoder;

impl Encoder {
    pub fn encode(input: &[u8], original_size: u64, lz77_size: u64) -> EncodedData {
        let tree = Node::build(input).expect("Must not be empty");

        let code_table = tree.build_code_table();

        let data = EncodedData::encode_bytes(input, &code_table);

        let header = Header::new(lz77_size, original_size, code_table);

        EncodedData { header, data }
    }
}
