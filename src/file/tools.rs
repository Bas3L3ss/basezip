use std::{ fs, path::{ Path, PathBuf } };
use crate::file::{ File, FileError };

pub fn open(path: impl AsRef<Path>) -> Result<File, FileError> {
    let path = path.as_ref();

    let contents = fs::read(&path).map_err(|source| FileError::Read {
        path: path.display().to_string(),
        source,
    })?;

    if contents.len() == 0 {
        return Err(FileError::EmptyFile);
    }

    Ok(File {
        path: path.display().to_string(),
        contents,
    })
}

pub fn write<T: AsRef<[u8]>>(path: impl AsRef<Path>, contents: T) -> Result<(), FileError> {
    let path = path.as_ref();

    std::fs
        ::write(path, contents)
        .map_err(|source| FileError::Write { path: path.display().to_string(), source })?;

    Ok(())
}

pub fn remove_bzip_extension(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    match path.extension() {
        Some(ext) if ext == "bzip" => { path.with_extension("") }
        _ => path.to_path_buf(),
    }
}
