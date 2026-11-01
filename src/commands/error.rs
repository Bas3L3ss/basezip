use thiserror::Error;

use crate::{ file::FileError };

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)] File(#[from] FileError),
}
