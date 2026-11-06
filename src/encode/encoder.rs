use crate::{ encode::Header, huffman::Node };
use super::EncodedData;

pub struct Encoder;

impl Encoder {
    pub fn encode(input: &[u8]) -> EncodedData {
        let tree = Node::build(input).expect("Must not be empty");

        let code_table = tree.build_code_table();

        let data = EncodedData::encode_bytes(input, &code_table);

        let header = Header::new(input.len() as u64, code_table);

        EncodedData { header, data }
    }
}
