use crate::{ commands::error::Error, compressor::huffman_tree, file::open };

pub fn compress(path: &str) -> Result<(), Error> {
    let file = open(path)?;
    let tree = huffman_tree::Node::from_bytes(&file.contents);
    dbg!(tree);
    Ok(())
}
