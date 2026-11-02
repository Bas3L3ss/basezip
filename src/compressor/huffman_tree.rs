use std::collections::HashMap;
use std::cmp::{ Reverse };

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

    /// Placeholder tree returned for empty input.
    fn empty() -> Self {
        Self::new(NodeKind::Internal { left: None, right: None }, 0)
    }

    /// Joins two nodes under a new internal node whose frequency is their sum.
    fn merge(left: Node, right: Node) -> Self {
        let freq = left.freq + right.freq;
        let kind = NodeKind::Internal {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        Self::new(kind, freq)
    }

    /// One leaf node per distinct byte in `contents`, with its frequency.
    fn leaves(contents: &[u8]) -> Vec<Node> {
        let mut freq_map: HashMap<u8, usize> = HashMap::new();
        for &byte in contents {
            *freq_map.entry(byte).or_insert(0) += 1;
        }

        freq_map
            .into_iter()
            .map(|(symbol, freq)| Node::new(NodeKind::Leaf { symbol: symbol as u16 }, freq))
            .collect()
    }

    /// Default entry point. Point this at whichever implementation wins the benchmark.
    pub fn build(contents: &[u8]) -> Self {
        Self::build_with_vec(contents)
    }

    /// Builds the tree with a `Vec` kept sorted in descending order (smallest at the end).
    pub fn build_with_vec(contents: &[u8]) -> Self {
        let mut nodes = Self::leaves(contents);
        if nodes.is_empty() {
            return Self::empty();
        }

        nodes.sort_by_key(|n| Reverse(n.freq));

        while nodes.len() > 1 {
            let left = nodes.pop().expect("len > 1");
            let right = nodes.pop().expect("len > 1");
            nodes.push(Self::merge(left, right));
            nodes.sort_by_key(|n| Reverse(n.freq));
        }

        nodes.pop().expect("non-empty")
    }

    /// Builds the tree with a min-heap keyed on frequency.
    pub fn build_with_heap(contents: &[u8]) -> Self {
        let mut nodes = Self::leaves(contents);
        if nodes.is_empty() {
            return Self::empty();
        }

        nodes.sort_by_key(|n| Reverse(n.freq));

        while nodes.len() > 1 {
            let left = nodes.pop().expect("len > 1");
            let right = nodes.pop().expect("len > 1");
            nodes.push(Self::merge(left, right));
            nodes.sort_by_key(|n| Reverse(n.freq));
        }

        nodes.pop().expect("non-empty")
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

        let tree = Node::build(&contents);

        assert_eq!(tree.freq, 123);

        let wpl = weighted_path_length(&tree, 0);

        assert_eq!(wpl, 169);
    }

    #[test]
    fn builds_tree_with_two_symbols() {
        let contents = b"AABB";
        let tree = Node::build(contents);

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
        let tree = Node::build(contents);

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
        let tree = Node::build(contents);

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
        let tree = Node::build(contents);

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
        let tree = Node::build(contents);

        assert_eq!(tree.freq, contents.len());
        assert_valid_tree(&tree);

        let mut symbols = Vec::new();
        collect_symbols(&tree, &mut symbols);
        symbols.sort_unstable();

        assert_eq!(symbols, vec![65, 66]);
    }
}
