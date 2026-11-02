use crate::huffman::Node;
use super::EncodedData;

pub struct Encoder;

impl Encoder {
    pub fn encode(input: &[u8]) -> EncodedData {
        // 1. Calculate symbol frequencies.
        // 2. Build Huffman tree.
        // 3. Generate Huffman code lengths.
        // 4. Generate canonical Huffman codes.
        // 5. Encode input into bytes.
        // 6. Build metadata.

        todo!()
    }

    fn calculate_frequencies(input: &[u8]) -> [usize; 256] {
        todo!()
    }

    fn build_tree(frequencies: &[usize; 256]) -> Option<Node> {
        todo!()
    }

    fn build_code_lengths(tree: &Node) -> [u8; 256] {
        todo!()
    }

    fn build_canonical_codes(code_lengths: &[u8; 256]) -> [Option<(u32, u8)>; 256] {
        todo!()
    }

    fn encode_symbols(input: &[u8], codes: &[Option<(u32, u8)>; 256]) -> Vec<u8> {
        todo!()
    }
}
