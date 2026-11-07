pub mod value;
pub mod window;
pub use value::*;
pub use window::*;

pub struct LZ77;

impl LZ77 {
    pub fn encode(contents: &[u8]) -> Vec<u8> {
        todo!()
    }

    pub fn compress(contents: &[u8]) -> Vec<Value> {
        let mut window = Window::new(WINDOW_CONTEXT as usize);

        for byte in contents {
            window.push(*byte);
            dbg!(&window.inner);
        }

        todo!()
    }
}
