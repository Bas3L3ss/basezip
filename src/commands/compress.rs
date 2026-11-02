use crate::{ commands::error::Error, file::open, huffman::huffman_tree };

pub fn compress(path: &str) -> Result<(), Error> {
    let file = open(path)?;
    let tree = huffman_tree::Node::build(&file.contents).expect("Must have nodes"); // TODO: HANDLE UNWRAP
    let code_table = tree.build_code_table();
    dbg!(code_table);
    Ok(())
}
