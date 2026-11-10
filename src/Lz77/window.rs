use std::{ collections::VecDeque };

use crate::Lz77::{ MAX_LENGTH, MIN_LENGTH, tool::hash };

pub const MAX_CHAIN_DEPTH: usize = 64;
const TRIPLE_HASH_SIZE: usize = 1 << 24;

#[derive(Debug, Clone, Copy)]
struct ChainEntry {
    position: usize,
    length: usize,
}

#[derive(Debug)]
pub struct Window {
    capacity: usize,
    history: VecDeque<u8>,
    triple_head: Vec<Option<ChainEntry>>,
    triple_prev: Vec<Option<usize>>,
}

impl Window {
    pub fn new(capacity: usize) -> Self {
        if capacity >= (MIN_LENGTH as usize) {
            Self {
                capacity,
                history: VecDeque::with_capacity(capacity),
                triple_head: vec![None; TRIPLE_HASH_SIZE],
                triple_prev: vec![None; capacity],
            }
        } else {
            panic!()
        }
    }

    pub fn push(&mut self, byte: u8, origin_position: usize) {
        if self.history.len() == self.capacity {
            let oldest_triple_hash = hash(
                &[self.history[0], self.history[1], self.history[2]]
            ) as usize;

            let head = self.triple_head[oldest_triple_hash].as_mut().unwrap();
            head.length -= 1;

            if head.length == 0 {
                self.triple_head[oldest_triple_hash] = None;
            }
            self.history.pop_front();
        }

        self.history.push_back(byte);

        if self.history.len() >= (MIN_LENGTH as usize) {
            let absolute_position = origin_position - ((MIN_LENGTH as usize) - 1);

            let start_of_last = self.history.len() - (MIN_LENGTH as usize);

            let hash = hash(
                &[
                    self.history[start_of_last],
                    self.history[start_of_last + 1],
                    self.history[start_of_last + 2],
                ]
            ) as usize;

            let mut length = 1;
            let prev_absolute_position = match self.triple_head[hash] {
                None => { None }
                Some(p) => {
                    length += p.length;
                    Some(p.position)
                }
            };

            let relative_position = absolute_position % self.capacity;

            self.triple_prev[relative_position] = prev_absolute_position;
            self.triple_head[hash] = Some(ChainEntry { position: absolute_position, length });
        }
    }

    pub fn find_longest_match(
        &self,
        original: &[u8],
        remainings: &[u8],
        remaining_absolute_start: usize,
        max_chain_depth: usize
    ) -> Option<(u16, u16)> {
        if remainings.len() < (MIN_LENGTH as usize) {
            return None;
        }

        let mut distance: u16 = 0;
        let mut length: u16 = 0;
        let max_length: u16 = remainings.len().min(MAX_LENGTH.into()) as u16;

        if let Some(head) = self.triple_head[hash(&remainings[0..MIN_LENGTH as usize]) as usize] {
            let mut step = 0;
            let mut cur = head.position;

            loop {
                let mut current_length: u16 = MIN_LENGTH;
                let new_distance: u16 = (remaining_absolute_start - cur) as u16;

                loop {
                    if current_length < max_length {
                        let source_idx =
                            cur + ((current_length as usize) % (new_distance as usize));

                        if remainings[current_length as usize] == original[source_idx] {
                            current_length += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if current_length > length {
                    distance = new_distance;
                    length = current_length;

                    if current_length == max_length {
                        return Some((distance, length));
                    }
                }

                step += 1;

                if step < head.length && step < max_chain_depth {
                    cur = self.triple_prev[cur % self.capacity].expect("Invalid traversal");
                } else {
                    break;
                }
            }
        }

        if length < MIN_LENGTH {
            return None;
        }

        Some((distance, length))
    }

    pub fn extend(&mut self, input: &[u8], start: usize) {
        for (pos, &byte) in input.iter().enumerate() {
            self.push(byte, pos + start);
        }
    }
}
