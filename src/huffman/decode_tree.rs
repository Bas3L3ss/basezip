use crate::{ decode::DecompressError, huffman::CodeTable };

#[derive(Debug, PartialEq)]
pub struct DecodeTree {
    pub arena: Vec<DecodeNode>,
    pub root: usize,
}

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
        left: Option<usize>,
        right: Option<usize>,
    },
}

impl DecodeTree {
    pub fn build_decode_tree(code_table: CodeTable) -> Result<Self, DecompressError> {
        let mut arena = vec![
            DecodeNode::new(DecodeNodeKind::Internal {
                left: None,
                right: None,
            })
        ];

        for (symbol, entry) in code_table.iter().enumerate() {
            let Some((code, length)) = entry else {
                continue;
            };

            if *length == 0 || *length > 32 {
                return Err(DecompressError::InvalidCodeLength);
            }

            let mut current = 0;

            for bit_index in (0..*length).rev() {
                let bit = (code >> bit_index) & 1;

                let child_index = match &arena[current].kind {
                    DecodeNodeKind::Internal { left, right } => {
                        if bit == 0 { *left } else { *right }
                    }
                    DecodeNodeKind::Leaf { .. } => {
                        return Err(DecompressError::UnexpectedLeaf);
                    }
                };

                current = match child_index {
                    Some(index) => index,
                    None => {
                        let new_child = arena.len();

                        arena.push(
                            DecodeNode::new(DecodeNodeKind::Internal {
                                left: None,
                                right: None,
                            })
                        );

                        let DecodeNodeKind::Internal { left, right } =
                            &mut arena[current].kind else {
                            return Err(DecompressError::UnexpectedLeaf);
                        };

                        let child = if bit == 0 { left } else { right };
                        *child = Some(new_child);

                        new_child
                    }
                };
            }

            match &arena[current].kind {
                DecodeNodeKind::Internal { left, right } if left.is_none() && right.is_none() => {
                    arena[current].kind = DecodeNodeKind::Leaf {
                        symbol: symbol as u16,
                    };
                }
                _ => {
                    return Err(DecompressError::UnexpectedLeaf);
                }
            }
        }

        Ok(Self { arena, root: 0 })
    }
}

impl DecodeNode {
    pub fn new(kind: DecodeNodeKind) -> Self {
        Self { kind }
    }
}
