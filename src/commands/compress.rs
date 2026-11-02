use crate::{ commands::error::Error, compressor::huffman_tree, file::open };

pub fn compress(path: &str) -> Result<(), Error> {
    let file = open(path)?;
    let tree = huffman_tree::Node::build(&file.contents).unwrap(); // TODO: HANDLE UNWRAP
    let code_table = tree.build_code_table();
    dbg!(code_table);
    Ok(())
}
