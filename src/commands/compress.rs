use crate::{ Lz77::LZ77, commands::error::Error, encode::Encoder, file::{ open, write } };

pub fn compress(path: &str) -> Result<(), Error> {
    let file = open(path)?;

    let lz77 = LZ77::encode(&file.contents);
    let encoded_data = Encoder::encode(&lz77, file.size, lz77.len() as u64);
    let data = encoded_data.to_bytes();
    write(format!("{}.bzip", file.path), &data)?;
    println!("Compressed file is stored at: {}", format!("{}.bzip", file.path));
    Ok(())
}
