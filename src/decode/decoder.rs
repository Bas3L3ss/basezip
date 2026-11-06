use super::DecompressError;
use crate::{ encode::{ HEADER_LENGTH, Header }, huffman::{ DecodeNode, DecodeNodeKind } };

pub struct Decoder;

impl Decoder {
    pub fn decode(input: &[u8]) -> Result<String, DecompressError> {
        if input.len() < HEADER_LENGTH {
            return Err(DecompressError::IncorrectHeaderLength);
        }

        let header_slice = &input[0..HEADER_LENGTH];
        let input_slice = &input[HEADER_LENGTH..];

        if input_slice.len() == 0 {
            eprintln!("File is empty, there's nothing to decompress");
            return Ok(String::new());
        }

        // parse header
        let header = Header::deserialize(header_slice)?;

        // build huffman decode tree

        let tree = DecodeNode::build_decode_tree(header.code_table)?;

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
                    _ => {
                        return Err(DecompressError::UnexpectedTraversal);
                    }
                };

                let child = match child {
                    Some(c) => c,
                    None => {
                        return Err(DecompressError::UnexpectedTraversal);
                    }
                };

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
            eprintln!(
                "The decompression progress might have produced a different file due to difference in size..."
            );
        }

        Ok(
            std::str
                ::from_utf8(&out)
                .map_err(|_| DecompressError::FailedToParseString)?
                .to_string()
        )
    }
}
