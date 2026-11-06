use crate::{ commands::error::Error, decode::decoder::Decoder, file::{ open, write } };

pub fn decompress(path: &str) -> Result<(), Error> {
    let file = open(path)?;

    let original_contents = Decoder::decode(&file.contents);

    dbg!(original_contents);

    Ok(())
}
