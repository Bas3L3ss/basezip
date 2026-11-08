use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecompressError {
    #[error("Incorrect header length")] IncorrectHeaderLength,
    #[error("Invalid magic word")] InvalidMagicWord,
    #[error("Invalid code length")] InvalidCodeLength,
    #[error("Failed to parse {type_num}")] FailedToParseNumber {
        type_num: &'static str,
    },
    #[error("Reached an invalid state of the tree")] UnexpectedLeaf,
    #[error("Reached an invalid state while traversing")] UnexpectedTraversal,
    #[error("Reached an invalid state while decoding LZ77 signature")] UnexpectedValueSignature,
}
