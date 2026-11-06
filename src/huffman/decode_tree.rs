use crate::huffman::CodeTable;

#[derive(Debug, PartialEq)]
pub struct DecodeNode {
    kind: DecodeNodeKind,
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
    pub fn build_decode_tree(code_table: CodeTable) -> Self {
        fn dfs(cur_node: &mut DecodeNode, entry: (u32, u8), symbol: u16, current: isize) {
            let (code_byte, _) = entry;

            if current < 0 {
                match &cur_node.kind {
                    DecodeNodeKind::Internal { left, right } if
                        left.is_none() &&
                        right.is_none()
                    => {
                        cur_node.kind = DecodeNodeKind::Leaf { symbol };
                        return;
                    }
                    _ => panic!("Invalid leaf"), // TODO: Panic
                }
            } else {
                let bit = ((code_byte >> current) & 1) as u8;

                let (left, right) = match &mut cur_node.kind {
                    DecodeNodeKind::Leaf { .. } => {
                        panic!("invalid tree"); // TODO: Panic
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

                dfs(child.as_mut().unwrap(), entry, symbol, current - 1) // TODO: Handle unwrap
            }
        }

        let mut root = Self::new(DecodeNodeKind::Internal { left: None, right: None });
        for (symbol, entry) in code_table.iter().enumerate() {
            if let Some(entry) = entry {
                dfs(&mut root, *entry, symbol as u16, entry.1 as isize);
            }
        }
        root
    }

    pub fn new(kind: DecodeNodeKind) -> Self {
        Self { kind }
    }
}
