use crate::{
    commands::error::Error,
    decode::decoder::Decoder,
    file::{ open, remove_bzip_extension, write },
};

pub fn decompress(path: &str) -> Result<(), Error> {
    let file = open(path)?;

    let original_contents = Decoder::decode(&file.contents);

    let resolved_path = remove_bzip_extension(&path);

    write(resolved_path, original_contents)?;

    Ok(())
}
