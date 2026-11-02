use std::{ fs, path::Path };
use crate::file::{ File, FileError };

pub fn open(path: impl AsRef<Path>) -> Result<File, FileError> {
    let path = path.as_ref();

    let contents = fs::read(path).map_err(|source| FileError::Read {
        path: path.display().to_string(),
        source,
    })?;

    let folder_path = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_string_lossy()
        .into_owned();

    Ok(File {
        folder_path,
        contents,
    })
}
