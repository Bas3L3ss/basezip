use crate::{ commands::error::Error, file::open };

pub fn compress(path: &str) -> Result<(), Error> {
    let content = open(path)?;
    dbg!(&content);
    Ok(())
}
