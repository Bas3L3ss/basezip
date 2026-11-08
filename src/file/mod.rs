pub mod error;
pub mod tools;

pub use error::*;
pub use tools::*;

#[derive(Debug)]
pub struct File {
    pub path: String,
    pub size: u64,
    pub contents: Vec<u8>,
}
