use crate::{ commands::error::Error, compressor::huffman_tree, file::open };

pub fn compress(path: &str) -> Result<(), Error> {
    let file = open(path)?;
    let tree = huffman_tree::Node::build(&file.contents);
    dbg!(tree);
    Ok(())
}
