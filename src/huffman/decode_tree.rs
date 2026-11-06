use crate::{ decode::DecompressError, huffman::CodeTable };

#[derive(Debug, PartialEq)]
pub struct DecodeNode {
    pub kind: DecodeNodeKind,
}

#[derive(Debug, PartialEq)]
pub enum DecodeNodeKind {
    Leaf {
        symbol: u16,
    },
    Internal {
        left: Option<Box<DecodeNode>>,
        right: Option<Box<DecodeNode>>,
    },
}

impl DecodeNode {
    pub fn build_decode_tree(code_table: CodeTable) -> Result<Self, DecompressError> {
        fn dfs(
            cur_node: &mut DecodeNode,
            code_byte: u32,
            symbol: u16,
            current: isize
        ) -> Result<(), DecompressError> {
            if current < 0 {
                match &cur_node.kind {
                    DecodeNodeKind::Internal { left, right } if
                        left.is_none() &&
                        right.is_none()
                    => {
                        cur_node.kind = DecodeNodeKind::Leaf { symbol };
                        Ok(())
                    }
                    _ => {
                        return Err(DecompressError::UnexpectedLeaf);
                    }
                }
            } else {
                let bit = ((code_byte >> current) & 1) as u8;

                let (left, right) = match &mut cur_node.kind {
                    DecodeNodeKind::Leaf { .. } => {
                        return Err(DecompressError::UnexpectedLeaf);
                    }
                    DecodeNodeKind::Internal { left, right } => (left, right),
                };

                let child = if bit == 0 { left } else { right };

                if child.is_none() {
                    *child = Some(
                        Box::new(
                            DecodeNode::new(DecodeNodeKind::Internal {
                                left: None,
                                right: None,
                            })
                        )
                    );
                }

                dfs(
                    child.as_mut().expect("Unreachable as child is created above"),
                    code_byte,
                    symbol,
                    current - 1
                )
            }
        }

        let mut root = Self::new(DecodeNodeKind::Internal { left: None, right: None });
        for (symbol, entry) in code_table.iter().enumerate() {
            if let Some(entry) = entry {
                dfs(&mut root, entry.0, symbol as u16, (entry.1 - 1) as isize)?;
            }
        }
        Ok(root)
    }

    pub fn new(kind: DecodeNodeKind) -> Self {
        Self { kind }
    }
}
