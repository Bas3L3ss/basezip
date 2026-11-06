use crate::{ commands::error::Error, encode::Encoder, file::{ open, write } };

pub fn compress(path: &str) -> Result<(), Error> {
    let file = open(path)?;
    let encoded_data = Encoder::encode(&file.contents);
    let data = encoded_data.to_bytes();
    write(format!("{}.bzip", file.path), &data)?;

    println!("File compressed successfully in: {}", format!("{}.bzip", file.path));
    Ok(())
}
