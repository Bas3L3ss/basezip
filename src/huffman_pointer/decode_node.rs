use crate::{ decode::DecompressError, huffman_pointer::CodeTable };

#[derive(Debug, PartialEq)]
pub struct PointerDecodeNode {
    pub kind: PointerDecodeNodeKind,
}

#[derive(Debug, PartialEq)]
pub enum PointerDecodeNodeKind {
    Leaf {
        symbol: u16,
    },
    Internal {
        left: Option<Box<PointerDecodeNode>>,
        right: Option<Box<PointerDecodeNode>>,
    },
}

impl PointerDecodeNode {
    pub fn build_decode_tree(code_table: CodeTable) -> Result<Self, DecompressError> {
        let mut root = Self::new(PointerDecodeNodeKind::Internal {
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
                    PointerDecodeNodeKind::Leaf { .. } => {
                        return Err(DecompressError::UnexpectedLeaf);
                    }
                    PointerDecodeNodeKind::Internal { left, right } => (left, right),
                };

                let child = if bit == 0 { left } else { right };

                current = child
                    .get_or_insert_with(|| {
                        Box::new(
                            Self::new(PointerDecodeNodeKind::Internal {
                                left: None,
                                right: None,
                            })
                        )
                    })
                    .as_mut();
            }

            match &current.kind {
                PointerDecodeNodeKind::Internal { left, right } if
                    left.is_none() &&
                    right.is_none()
                => {
                    current.kind = PointerDecodeNodeKind::Leaf {
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

    pub fn new(kind: PointerDecodeNodeKind) -> Self {
        Self { kind }
    }
}
