use thiserror::Error;

use crate::{ decode::DecompressError, file::FileError };

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)] File(#[from] FileError),
    #[error(transparent)] DecompressError(#[from] DecompressError),
}
