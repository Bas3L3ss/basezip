use crate::{
    decode::DecompressError,
    encode::{ HEADER_LENGTH, Header },
    huffman::{ DecodeNodeKind, DecodeTree },
};

pub struct Decoder;

impl Decoder {
    pub fn decode(input: &[u8]) -> Result<Vec<u8>, DecompressError> {
        let (input_slice, header) = Decoder::prep(input)?;

        if input_slice.is_empty() {
            eprintln!("File is empty, there's nothing to decompress");
            if header.original_size == 0 {
                return Ok(vec![]);
            }

            return Err(DecompressError::UnexpectedTraversal);
        }

        Self::with_arena_tree(input_slice, header)
    }

    pub fn prep(input: &[u8]) -> Result<(&[u8], Header), DecompressError> {
        if input.len() < HEADER_LENGTH {
            return Err(DecompressError::IncorrectHeaderLength);
        }

        let header_slice = &input[0..HEADER_LENGTH];

        let input_slice = &input[HEADER_LENGTH..];
        let header = Header::deserialize(header_slice)?;

        Ok((input_slice, header))
    }

    pub fn with_arena_tree(input_slice: &[u8], header: Header) -> Result<Vec<u8>, DecompressError> {
        let tree = DecodeTree::build_decode_tree(header.code_table)?;

        let arena = tree.arena;
        let root = tree.root;

        let mut out: Vec<u8> = Vec::with_capacity(header.original_size as usize);
        let mut current = root;

        'bytes: for byte in input_slice {
            for cur in (0..8).rev() {
                let bit = (byte >> cur) & 1;

                let child = match &arena[current].kind {
                    DecodeNodeKind::Internal { left, right } => {
                        if bit == 0 { left } else { right }
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

                match &arena[*child].kind {
                    DecodeNodeKind::Leaf { symbol } => {
                        out.push(*symbol as u8);
                        current = root;

                        if (out.len() as u64) == header.original_size {
                            break 'bytes;
                        }
                    }
                    DecodeNodeKind::Internal { .. } => {
                        current = *child;
                    }
                }
            }
        }
        if (out.len() as u64) != header.original_size {
            eprintln!(
                "The decompression progress might have produced a different file due to difference in size..."
            );
        }

        Ok(out)
    }
}
