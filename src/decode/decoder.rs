use crate::{ encode::{ HEADER_LENGTH, Header }, huffman::{ DecodeNode, DecodeNodeKind } };

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

        let mut out: Vec<u8> = vec![];

        let mut last_position = &tree;

        'bytes: for byte in input_slice {
            for cur in (0..8).rev() {
                let bit = (byte >> cur) & 1;

                let child = match &last_position.kind {
                    DecodeNodeKind::Internal { left, right } => {
                        if bit == 0 { left.as_deref() } else { right.as_deref() }
                    }
                    _ => { panic!("Unexpected travesal") } // TODO: handle panic
                };

                let child = child.unwrap(); // TODO: handle panic

                match &child.kind {
                    DecodeNodeKind::Leaf { symbol } => {
                        out.push(*symbol as u8);
                        last_position = &tree;

                        if (out.len() as u64) == header.original_size {
                            break 'bytes;
                        }
                    }
                    DecodeNodeKind::Internal { .. } => {
                        last_position = child;
                    }
                }
            }
        }
        if (out.len() as u64) != header.original_size {
            panic!("Odd compression.."); // TODO: handle panic
        }

        std::str::from_utf8(&out).unwrap().to_string() // TODO: Handle panic
    }
}
