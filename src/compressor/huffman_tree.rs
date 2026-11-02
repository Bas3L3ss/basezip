use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub freq: usize,
    pub kind: NodeKind,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Leaf {
        symbol: u16,
    },
    Internal {
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    },
}

impl Node {
    pub fn new(kind: NodeKind, freq: usize) -> Self {
        Self { kind, freq }
    }

    pub fn from_bytes(contents: &[u8]) -> Self {
        let mut cursor = 0;
        let mut freq_map: HashMap<u8, usize> = HashMap::new();

        while cursor < contents.len() {
            let key = contents[cursor];
            freq_map
                .entry(key)
                .and_modify(|v| {
                    *v += 1;
                })
                .or_insert(1);
            cursor += 1;
        }

        if freq_map.is_empty() {
            return Self { freq: 0, kind: NodeKind::Internal { left: None, right: None } };
        }

        let mut freq_vec: Vec<Node> = freq_map
            .iter()
            .map(|(symbol, freq)| Node::new(NodeKind::Leaf { symbol: *symbol as u16 }, *freq))
            .collect();

        freq_vec.sort_by(|a, b| a.freq.cmp(&b.freq).reverse());

        while freq_vec.len() > 1 {
            let left_node = freq_vec.pop().expect("algo failed");
            let mut freq = left_node.freq;

            if let Some(right_node) = freq_vec.pop() {
                freq += right_node.freq;
                let kind = NodeKind::Internal {
                    left: Some(Box::new(left_node)),
                    right: Some(Box::new(right_node)),
                };

                freq_vec.push(Node::new(kind, freq));
            } else {
                panic!("algo failed");
                break;
            }

            freq_vec.sort_by(|a, b| a.freq.cmp(&b.freq).reverse());
        }

        freq_vec[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weighted_path_length(node: &Node, depth: usize) -> usize {
        match &node.kind {
            NodeKind::Leaf { .. } => node.freq * depth,

            NodeKind::Internal { left, right } => {
                weighted_path_length(left.as_ref().unwrap(), depth + 1) +
                    weighted_path_length(right.as_ref().unwrap(), depth + 1)
            }
        }
    }

    fn assert_valid_tree(node: &Node) -> usize {
        match &node.kind {
            NodeKind::Leaf { .. } => {
                assert!(node.freq > 0);
                node.freq
            }
            NodeKind::Internal { left, right } => {
                let left = left.as_ref().expect("internal node missing left child");
                let right = right.as_ref().expect("internal node missing right child");

                let left_freq = assert_valid_tree(left);
                let right_freq = assert_valid_tree(right);

                assert_eq!(
                    node.freq,
                    left_freq + right_freq,
                    "internal node frequency is incorrect"
                );

                node.freq
            }
        }
    }

    fn collect_symbols(node: &Node, symbols: &mut Vec<u16>) {
        match &node.kind {
            NodeKind::Leaf { symbol } => symbols.push(*symbol),
            NodeKind::Internal { left, right } => {
                collect_symbols(left.as_ref().unwrap(), symbols);
                collect_symbols(right.as_ref().unwrap(), symbols);
            }
        }
    }

    #[test]
    fn produces_optimal_huffman_tree() {
        let mut contents = Vec::new();

        for (symbol, frequency) in [
            (b'A', 5),
            (b'B', 5),
            (b'C', 6),
            (b'D', 7),
            (b'E', 100),
        ] {
            contents.extend(std::iter::repeat(symbol).take(frequency));
        }

        let tree = Node::from_bytes(&contents);

        assert_eq!(tree.freq, 123);

        let wpl = weighted_path_length(&tree, 0);

        assert_eq!(wpl, 169);
    }

    #[test]
    fn builds_tree_with_two_symbols() {
        let contents = b"AABB";
        let tree = Node::from_bytes(contents);

        assert_eq!(tree.freq, contents.len());
        assert_valid_tree(&tree);

        let mut symbols = Vec::new();
        collect_symbols(&tree, &mut symbols);
        symbols.sort_unstable();

        assert_eq!(symbols, vec![65, 66]);
    }

    #[test]
    fn builds_tree_with_multiple_symbols() {
        let contents = b"AAAABBCCD";
        let tree = Node::from_bytes(contents);

        assert_eq!(tree.freq, contents.len());
        assert_valid_tree(&tree);

        let mut symbols = Vec::new();
        collect_symbols(&tree, &mut symbols);
        symbols.sort_unstable();

        assert_eq!(symbols, vec![65, 66, 67, 68]);
    }

    #[test]
    fn builds_tree_with_repeated_internal_merges() {
        let contents = b"AAAAAAAAAABBBBBCCCCDDDEEFG";
        let tree = Node::from_bytes(contents);

        assert_eq!(tree.freq, contents.len());
        assert_valid_tree(&tree);

        let mut symbols = Vec::new();
        collect_symbols(&tree, &mut symbols);
        symbols.sort_unstable();

        assert_eq!(symbols, vec![65, 66, 67, 68, 69, 70, 71]);
    }

    #[test]
    fn builds_tree_with_equal_frequencies() {
        let contents = b"AABBCCDDEEFFGGHH";
        let tree = Node::from_bytes(contents);

        assert_eq!(tree.freq, contents.len());
        assert_valid_tree(&tree);

        let mut symbols = Vec::new();
        collect_symbols(&tree, &mut symbols);
        symbols.sort_unstable();

        assert_eq!(symbols, vec![65, 66, 67, 68, 69, 70, 71, 72]);
    }

    #[test]
    fn builds_tree_with_skewed_frequencies() {
        let contents = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB";
        let tree = Node::from_bytes(contents);

        assert_eq!(tree.freq, contents.len());
        assert_valid_tree(&tree);

        let mut symbols = Vec::new();
        collect_symbols(&tree, &mut symbols);
        symbols.sort_unstable();

        assert_eq!(symbols, vec![65, 66]);
    }
}
