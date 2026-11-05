use crate::{ commands::error::Error, encode::Encoder, file::open };

pub fn compress(path: &str) -> Result<(), Error> {
    let file = open(path)?;
    let encoded_data = Encoder::encode(&file.contents);
    dbg!(encoded_data);
    Ok(())
}
