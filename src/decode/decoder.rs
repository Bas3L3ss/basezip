use crate::{
    Lz77::{ Signature },
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
            if header.lz77_size == 0 {
                return Ok(vec![]);
            }

            return Err(DecompressError::UnexpectedTraversal);
        }

        let values_bytes = Self::to_lz77_with_arena_tree(input_slice, header)?;
        Self::values_to_orginal(&values_bytes, &header)
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

    pub fn to_lz77_with_arena_tree(
        input_slice: &[u8],
        header: Header
    ) -> Result<Vec<u8>, DecompressError> {
        let tree = DecodeTree::build_decode_tree(header.code_table)?;

        let arena = tree.arena;
        let root = tree.root;

        let mut out: Vec<u8> = Vec::with_capacity(header.lz77_size as usize);
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

                        if (out.len() as u64) == header.lz77_size {
                            break 'bytes;
                        }
                    }
                    DecodeNodeKind::Internal { .. } => {
                        current = *child;
                    }
                }
            }
        }

        if (out.len() as u64) != header.lz77_size {
            return Err(DecompressError::UnexpectedTraversal);
        }

        Ok(out)
    }

    pub fn values_to_orginal(input: &[u8], header: &Header) -> Result<Vec<u8>, DecompressError> {
        let mut out = Vec::with_capacity(header.original_size as usize);
        let mut position = 0;

        while position < input.len() {
            let signature: Signature = Signature::check(input[position]).ok_or(
                DecompressError::UnexpectedValueSignature
            )?;

            match signature.bits {
                0b0 => {
                    position += 1;

                    let symbol = *input.get(position).ok_or(DecompressError::UnexpectedTraversal)?;

                    out.push(symbol);
                    position += 1;
                }

                0b10 => {
                    position += 1;

                    let distance_bytes: [u8; 2] = input
                        .get(position..position + 2)
                        .ok_or(DecompressError::FailedToParseNumber {
                            type_num: "u16",
                        })?
                        .try_into()
                        .unwrap();

                    let distance = u16::from_le_bytes(distance_bytes);
                    position += 2;

                    let length_bytes: [u8; 2] = input
                        .get(position..position + 2)
                        .ok_or(DecompressError::FailedToParseNumber {
                            type_num: "u16",
                        })?
                        .try_into()
                        .unwrap();

                    let length = u16::from_le_bytes(length_bytes);
                    position += 2;

                    if distance == 0 || (distance as usize) > out.len() {
                        return Err(DecompressError::UnexpectedTraversal);
                    }

                    let remaining = (header.original_size as usize).saturating_sub(out.len());

                    if (length as usize) > remaining {
                        return Err(DecompressError::UnexpectedTraversal);
                    }

                    for _ in 0..length {
                        let source = out.len() - (distance as usize);
                        let byte = out[source];

                        out.push(byte);
                    }
                }

                _ => {
                    return Err(DecompressError::UnexpectedValueSignature);
                }
            }
        }

        if (out.len() as u64) != header.original_size {
            return Err(DecompressError::UnexpectedTraversal);
        }

        Ok(out)
    }
}
