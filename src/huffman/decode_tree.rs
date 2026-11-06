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
        let mut root = Self::new(DecodeNodeKind::Internal {
            left: None,
            right: None,
        });

        for (symbol, entry) in code_table.iter().enumerate() {
            let Some((code, length)) = entry else {
                continue;
            };

            if *length == 0 || *length > 32 {
                return Err(DecompressError::InvalidCodeLength);
            }

            let mut current = &mut root;

            for bit_index in (0..*length).rev() {
                let bit = (code >> bit_index) & 1;

                let (left, right) = match &mut current.kind {
                    DecodeNodeKind::Leaf { .. } => {
                        return Err(DecompressError::UnexpectedLeaf);
                    }
                    DecodeNodeKind::Internal { left, right } => (left, right),
                };

                let child = if bit == 0 { left } else { right };

                current = child
                    .get_or_insert_with(|| {
                        Box::new(
                            Self::new(DecodeNodeKind::Internal {
                                left: None,
                                right: None,
                            })
                        )
                    })
                    .as_mut();
            }

            match &current.kind {
                DecodeNodeKind::Internal { left, right } if left.is_none() && right.is_none() => {
                    current.kind = DecodeNodeKind::Leaf {
                        symbol: symbol as u16,
                    };
                }
                _ => {
                    return Err(DecompressError::UnexpectedLeaf);
                }
            }
        }

        Ok(root)
    }

    pub fn new(kind: DecodeNodeKind) -> Self {
        Self { kind }
    }
}
