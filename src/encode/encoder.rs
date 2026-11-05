use crate::{ encode::Metadata, huffman::Node };
use super::EncodedData;

pub struct Encoder;

impl Encoder {
    pub fn encode(input: &[u8]) -> EncodedData {
        let tree = Node::build(input).expect("Must not be empty");

        let code_table = tree.build_code_table();

        let encoded_input = EncodedData::encode_bytes(input, &code_table);

        let metadata = Metadata::new(input.len() as u64, code_table);

        todo!()
    }

    fn build_code_lengths(tree: &Node) -> [u8; 256] {
        todo!()
    }

    fn build_canonical_codes(code_lengths: &[u8; 256]) -> [Option<(u32, u8)>; 256] {
        todo!()
    }

    fn encode_symbols(input: &[u8], codes: &[Option<(u32, u8)>; 256]) -> Vec<u8> {
        todo!()
    }
}
