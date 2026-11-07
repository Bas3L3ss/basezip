pub const MAX_DISTANCE: u16 = 32 * 1024; // 32KIB
pub const MAX_LENGTH: u16 = 258;
pub const MIN_LENGTH: u16 = 3;
// pub const WINDOW_CONTEXT: u16 = 32 * 1024; // 32KIB
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
}
