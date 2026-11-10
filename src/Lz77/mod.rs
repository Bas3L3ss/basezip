pub mod value;
pub mod signature;
pub mod window;
pub use value::*;
pub use signature::*;
pub use window::*;
pub mod tool;

pub struct LZ77;

impl LZ77 {
    pub fn encode(input: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut window = Window::new(WINDOW_CONTEXT as usize);

        let mut pos = 0;

        while pos < input.len() {
            let remaining = &input[pos..];

            if
                let Some((distance, length)) = window.find_longest_match(
                    input,
                    remaining,
                    pos,
                    MAX_CHAIN_DEPTH
                )
            {
                // Emit match
                let val: Value = Value::match_value(distance, length);
                val.to_bytes(&mut out);

                // Add matched input bytes to history
                window.extend(&input[pos..pos + (length as usize)], pos);

                // Advance by match length
                pos += length as usize;
            } else {
                // Emit literal
                let val = Value::literal_value(input[pos]);
                val.to_bytes(&mut out);

                // Add current byte to history
                window.push(input[pos], pos);

                // Advance by one
                pos += 1;
            }
        }

        out
    }
}
