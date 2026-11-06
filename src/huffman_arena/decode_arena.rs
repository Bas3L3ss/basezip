use crate::{ decode::DecompressError, huffman_pointer::CodeTable };

#[derive(Debug, PartialEq)]
pub struct DecodeArena {
    pub arena: Vec<ArenaDecodeNode>,
    pub root: usize,
}

#[derive(Debug, PartialEq)]
pub struct ArenaDecodeNode {
    pub kind: ArenaDecodeNodeKind,
}

#[derive(Debug, PartialEq)]
pub enum ArenaDecodeNodeKind {
    Leaf {
        symbol: u16,
    },
    Internal {
        left: Option<usize>,
        right: Option<usize>,
    },
}

impl DecodeArena {
    pub fn build_decode_tree(code_table: CodeTable) -> Result<Self, DecompressError> {
        let mut arena = vec![
            ArenaDecodeNode::new(ArenaDecodeNodeKind::Internal {
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
                    ArenaDecodeNodeKind::Internal { left, right } => {
                        if bit == 0 { *left } else { *right }
                    }
                    ArenaDecodeNodeKind::Leaf { .. } => {
                        return Err(DecompressError::UnexpectedLeaf);
                    }
                };

                current = match child_index {
                    Some(index) => index,
                    None => {
                        let new_child = arena.len();

                        arena.push(
                            ArenaDecodeNode::new(ArenaDecodeNodeKind::Internal {
                                left: None,
                                right: None,
                            })
                        );

                        let ArenaDecodeNodeKind::Internal { left, right } =
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
                ArenaDecodeNodeKind::Internal { left, right } if
                    left.is_none() &&
                    right.is_none()
                => {
                    arena[current].kind = ArenaDecodeNodeKind::Leaf {
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

impl ArenaDecodeNode {
    pub fn new(kind: ArenaDecodeNodeKind) -> Self {
        Self { kind }
    }
}
