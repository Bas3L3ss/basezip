pub mod error;
pub mod tools;

pub use error::*;
pub use tools::*;

#[derive(Debug)]
pub struct File {
    pub folder_path: String,
    pub content: Vec<u8>,
}
