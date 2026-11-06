pub mod error;
pub mod tools;

pub use error::*;
pub use tools::*;

#[derive(Debug)]
pub struct File {
    pub path: String,
    pub contents: Vec<u8>,
}
