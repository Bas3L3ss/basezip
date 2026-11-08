pub mod value;
pub mod window;
pub mod signature;
pub use value::*;
pub use signature::*;
pub use window::*;

pub struct LZ77;

impl LZ77 {
    pub fn encode(contents: &[u8]) -> Vec<u8> {
        let values: Vec<Value> = Self::compress(contents); // TODO: Implement direct values encoding

        let mut out: Vec<u8> = Vec::new();

        for value in values {
            value.to_bytes(&mut out);
        }

        out
    }

    pub fn compress(input: &[u8]) -> Vec<Value> {
        let mut out = Vec::new();
        let mut window = Window::new(WINDOW_CONTEXT as usize);

        let mut pos = 0;
        while pos < input.len() {
            let remaining = &input[pos..];

            if let Some((distance, length)) = window.find_longest_match(remaining) {
                // Emit match
                let val: Value = Value::match_value(distance, length);
                out.push(val);

                // Add matched input bytes to history
                window.extend(&input[pos..pos + (length as usize)]);

                // Advance by match length
                pos += length as usize;
            } else {
                // Emit literal
                let val = Value::literal_value(input[pos]);
                out.push(val);

                // Add current byte to history
                window.push(input[pos]);

                // Advance by one
                pos += 1;
            }
        }
        out
    }
}
