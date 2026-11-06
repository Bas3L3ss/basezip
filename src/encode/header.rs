use crate::{ decode::DecompressError, huffman::CodeTable };

const CODETABLE_LENGTH: usize = 256;
const CODE_LENGTH_BYTES: usize = 1;
const CODE_BYTES: usize = 4;
const MAGIC_WORD: &[u8; 7] = b"BASEZIP";
pub const HEADER_LENGTH: usize =
    MAGIC_WORD.len() + 8 + CODETABLE_LENGTH * (CODE_LENGTH_BYTES + CODE_BYTES);

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub original_size: u64,
    pub code_table: CodeTable,
}

impl Header {
    pub fn new(original_size: u64, code_table: CodeTable) -> Self {
        Self { original_size, code_table }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_LENGTH);

        out.extend_from_slice(MAGIC_WORD);
        out.extend_from_slice(&self.original_size.to_le_bytes());

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
    pub fn deserialize(bytes: &[u8]) -> Result<Self, DecompressError> {
        if bytes.len() != HEADER_LENGTH {
            return Err(DecompressError::IncorrectHeaderLength);
        }

        let mut start = MAGIC_WORD.len();

        let magic_word = &bytes[..start];

        if magic_word != MAGIC_WORD {
            return Err(DecompressError::InvalidMagicWord);
        }

        let original_size = u64::from_le_bytes(
            bytes[start..start + 8]
                .try_into()
                .map_err(|_| DecompressError::FailedToParseNumber { type_num: "u8" })?
        );

        start += 8;

        let mut code_table: CodeTable = std::array::from_fn(|_| None);

        for entry in code_table.iter_mut() {
            let code_length = bytes[start];

            if code_length > 32 {
                return Err(DecompressError::InvalidCodeLength);
            }

            start += 1;

            let code = u32::from_le_bytes(
                bytes[start..start + 4]
                    .try_into()
                    .map_err(|_| DecompressError::FailedToParseNumber { type_num: "u32" })?
            );
            start += 4;

            if code_length != 0 {
                *entry = Some((code, code_length));
            }
        }

        Ok(Self {
            code_table,
            original_size,
        })
    }
}
