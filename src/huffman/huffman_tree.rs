use std::cmp::{ Reverse };

use super::min_heap::MinHeap;

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

#[derive(Debug)]
pub struct Node {
    freq: usize,
    kind: NodeKind,
}

#[derive(Debug)]
pub enum NodeKind {
    Leaf {
        symbol: u16,
    },
    Internal {
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    pub fn build_code_table(&self) -> [Option<Vec<bool>>; 256] {
        fn dfs(cur_node: &Node, out: &mut [Option<Vec<bool>>; 256], buf: &mut Vec<bool>) {
            match &cur_node.kind {
                NodeKind::Internal { left, right } => {
                    buf.push(false);
                    dfs(left, out, buf);
                    buf.pop();

                    buf.push(true);
                    dfs(right, out, buf);
                    buf.pop();
                }
                NodeKind::Leaf { symbol } => {
                    out[*symbol as usize] = Some(buf.clone());
                }
            }
        }

        let mut out: [Option<Vec<bool>>; 256] = std::array::from_fn(|_| None);
        let mut buf = Vec::new();

        if let NodeKind::Leaf { symbol } = &self.kind {
            out[*symbol as usize] = Some(vec![false]);
            return out;
        }

        dfs(self, &mut out, &mut buf);
        out
    }

    pub fn new(kind: NodeKind, freq: usize) -> Self {
        Self { kind, freq }
    }

    /// Joins two nodes under a new internal node whose frequency is their sum.
    fn merge(left: Node, right: Node) -> Self {
        let freq = left.freq.checked_add(right.freq).expect("frequency overflow");
        let kind = NodeKind::Internal {
            left: Box::new(left),
            right: Box::new(right),
        };
        Self::new(kind, freq)
    }

    /// One leaf node per distinct byte in `contents`, with its frequency.
    fn leaves(contents: &[u8]) -> Vec<Node> {
        let mut freq_map = [0usize; 256];

        for &byte in contents {
            freq_map[byte as usize] += 1;
        }

        freq_map
            .into_iter()
            .enumerate()
            .filter(|(_, freq)| *freq != 0)
            .map(|(symbol, freq)| Node::new(NodeKind::Leaf { symbol: symbol as u16 }, freq))
            .collect()
    }

    /// Default entry point. Point this at whichever implementation wins the benchmark.
    pub fn build(contents: &[u8]) -> Option<Self> {
        Self::build_with_heap(contents)
    }

    /// Builds the tree with a `Vec` kept sorted in descending order (smallest at the end).
    pub fn build_with_vec(contents: &[u8]) -> Option<Self> {
        let mut nodes = Self::leaves(contents);
        if nodes.is_empty() {
            return None;
        }

        nodes.sort_by_key(|n| Reverse(n.freq));

        while nodes.len() > 1 {
            let left = nodes.pop().expect("len > 1");
            let right = nodes.pop().expect("len > 1");
            nodes.push(Self::merge(left, right));
            nodes.sort_by_key(|n| Reverse(n.freq));
        }

        Some(nodes.pop().expect("non-empty"))
    }

    /// Builds the tree with a min-heap keyed on frequency.
    pub fn build_with_heap(contents: &[u8]) -> Option<Self> {
        let nodes = Self::leaves(contents);
        if nodes.is_empty() {
            return None;
        }
        let mut min_heap = MinHeap::default();
        for node in nodes {
            min_heap.insert(node);
        }

        while min_heap.inner.len() > 1 {
            let left = min_heap.pop_min().expect("len > 1");
            let right = min_heap.pop_min().expect("len > 1");
            min_heap.insert(Self::merge(left, right));
        }

        Some(min_heap.pop_min().expect("non-empty"))
    }

    pub fn frequency(&self) -> usize {
        self.freq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weighted_path_length(node: &Node, depth: usize) -> usize {
        match &node.kind {
            NodeKind::Leaf { .. } => node.freq * depth,

            NodeKind::Internal { left, right } => {
                weighted_path_length(left.as_ref(), depth + 1) +
                    weighted_path_length(right.as_ref(), depth + 1)
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
                let left = left.as_ref();
                let right = right.as_ref();

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
                collect_symbols(left.as_ref(), symbols);
                collect_symbols(right.as_ref(), symbols);
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

        let tree = Node::build(&contents).unwrap();

        assert_eq!(tree.freq, 123);

        let wpl = weighted_path_length(&tree, 0);

        assert_eq!(wpl, 169);
    }

    #[test]
    fn builds_tree_with_two_symbols() {
        let contents = b"AABB";
        let tree = Node::build(contents).unwrap();

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
        let tree = Node::build(contents).unwrap();

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
        let tree = Node::build(contents).unwrap();

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
        let tree = Node::build(contents).unwrap();

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
        let tree = Node::build(contents).unwrap();

        assert_eq!(tree.freq, contents.len());
        assert_valid_tree(&tree);

        let mut symbols = Vec::new();
        collect_symbols(&tree, &mut symbols);
        symbols.sort_unstable();

        assert_eq!(symbols, vec![65, 66]);
    }
}
