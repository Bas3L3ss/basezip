use std::collections::{ VecDeque };

use crate::Lz77::{ MAX_LENGTH, MIN_LENGTH };

pub struct Window {
    pub inner: VecDeque<u8>,
    capacity: usize,
}

impl Window {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, byte: u8) {
        if self.inner.len() == self.capacity {
            self.inner.pop_front();
        }

        self.inner.push_back(byte);
    }

    pub fn find_longest_match(&self, input: &[u8]) -> Option<(u16, u16)> {
        if input.len() < (MIN_LENGTH as usize) {
            return None;
        }

        let mut matches: Vec<usize> = Vec::new();

        for idx in 0..self.inner.len() {
            if self.inner[idx] == input[0] {
                matches.push(idx);
            }
        }

        if matches.is_empty() {
            return None;
        }

        let mut distance: u16 = 0;
        let mut length: u16 = 0;

        while let Some(starting_point) = matches.pop() {
            let mut current_length: u16 = 0;
            let new_distance: u16 = (self.inner.len() - starting_point) as u16;
            loop {
                if (current_length as usize) < input.len() && current_length < MAX_LENGTH {
                    let source_idx =
                        starting_point + ((current_length as usize) % (new_distance as usize));
                    if input[current_length as usize] == self.inner[source_idx] {
                        current_length += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if current_length > length {
                distance = (self.inner.len() - starting_point) as u16;
                length = current_length;
            }
        }

        if length < MIN_LENGTH {
            return None;
        }

        Some((distance, length))
    }

    pub fn extend(&mut self, input: &[u8]) {
        for &byte in input {
            self.push(byte);
        }
    }
}
