use crate::{ encode::{ HEADER_LENGTH, Header }, huffman::DecodeNode };

pub struct Decoder;

impl Decoder {
    pub fn decode(input: &[u8]) -> String {
        let header_slice = &input[0..HEADER_LENGTH];
        let input_slice = &input[HEADER_LENGTH..];

        // parse header
        let header = Header::deserialize(header_slice);

        // build huffman decode tree

        let tree = DecodeNode::build_decode_tree(header.code_table);

        // decode input

        todo!()
    }
}
