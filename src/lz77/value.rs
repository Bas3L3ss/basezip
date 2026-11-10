use crate::lz77::{ LITERAL_SIGNATURE, MATCH_SIGNATURE };

pub const MAX_LENGTH: u16 = 258;
pub const MIN_LENGTH: u16 = 3;
pub const WINDOW_CONTEXT: u16 = 32 * 1024; // 32KIB

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Literal(u8),
    Match {
        distance: u16,
        length: u16,
    },
}
impl Value {
    pub fn literal_value(symbol: u8) -> Self {
        Self::Literal(symbol)
    }

    pub fn match_value(distance: u16, length: u16) -> Self {
        Self::Match { distance, length }
    }
    pub fn to_bytes(&self, output: &mut Vec<u8>) {
        match *self {
            Value::Literal(symbol) => {
                output.push(LITERAL_SIGNATURE.bits); // TODO: IMPLEMENT BITWRITER TO WRITE IN THE EXACT BITS
                output.push(symbol);
            }

            Value::Match { distance, length } => {
                output.push(MATCH_SIGNATURE.bits);
                output.extend_from_slice(&distance.to_le_bytes());
                output.extend_from_slice(&length.to_le_bytes());
            }
        }
    }
}
