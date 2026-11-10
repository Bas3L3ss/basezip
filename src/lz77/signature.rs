use std::fmt::Debug;

pub struct Signature {
    pub bits: u8,
    pub length: u8,
}

pub const LITERAL_SIGNATURE: Signature = Signature {
    bits: 0b0,
    length: 1,
};

pub const MATCH_SIGNATURE: Signature = Signature {
    bits: 0b10,
    length: 2,
};

impl Debug for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0width$b}", self.bits, width = self.length as usize)
    }
}

impl Signature {
    pub fn check(byte: u8) -> Option<Self> {
        match byte {
            0b0 => { Some(LITERAL_SIGNATURE) }
            0b10 => { Some(MATCH_SIGNATURE) }
            _ => { None }
        }
    }
}
