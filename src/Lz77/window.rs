use std::{ collections::{ HashMap, VecDeque } };

use crate::Lz77::{ MAX_LENGTH, MIN_LENGTH };

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum HashKey {
    Byte(u8),
    Triple(u32),
}

#[derive(Debug)]
pub struct Window {
    capacity: usize,
    pub history: VecDeque<u8>,
    hashes_map: HashMap<HashKey, VecDeque<usize>>,
    hashes_vec: VecDeque<HashKey>,
}

impl Window {
    pub fn new(capacity: usize) -> Self {
        if capacity >= (MIN_LENGTH as usize) {
            Self {
                history: VecDeque::with_capacity(capacity),
                hashes_map: HashMap::new(),
                hashes_vec: VecDeque::with_capacity(capacity.saturating_sub(2)),
                capacity,
            }
        } else {
            panic!()
        }
    }

    fn hash(bytes: &[u8]) -> HashKey {
        match bytes.len() {
            1 => HashKey::Byte(bytes[0]),
            3 => {
                HashKey::Triple(
                    ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32)
                )
            }
            _ => unreachable!(), // TODO: Return error
        }
    }

    fn evict_oldest(&mut self) {
        let oldest_byte_hash = self.history.pop_front().unwrap();
        self.evict_hash(&HashKey::Byte(oldest_byte_hash));

        let oldest_hash = self.hashes_vec.pop_front().unwrap();
        self.evict_hash(&oldest_hash);
    }

    fn evict_hash(&mut self, hash: &HashKey) {
        let positions = self.hashes_map.get_mut(hash).unwrap(); // TODO: Handle unwrap
        positions.pop_front();

        if positions.is_empty() {
            self.hashes_map.remove(hash);
        }
    }

    fn insert_byte_hash(&mut self, byte: u8, position: usize) {
        let hash = Self::hash(&[byte]);
        self.hashes_map.entry(hash).or_default().push_back(position);
    }

    fn insert_hash(&mut self, origin_position: usize, offset: usize) {
        let start = self.history.len() - offset;

        let hash = Self::hash(
            &[self.history[start], self.history[start + 1], self.history[start + 2]]
        );
        self.hashes_vec.push_back(hash);
        self.hashes_map.entry(hash).or_default().push_back(origin_position);
    }

    pub fn push(&mut self, byte: u8, origin_position: usize) {
        if self.history.len() == self.capacity {
            self.evict_oldest();
        }

        self.history.push_back(byte);

        self.insert_byte_hash(byte, origin_position);

        if self.history.len() >= (MIN_LENGTH as usize) {
            let absolute_position = origin_position - ((MIN_LENGTH as usize) - 1);
            self.insert_hash(absolute_position, MIN_LENGTH as usize);
        }
    }

    pub fn find_longest_match(
        &self,
        original: &[u8],
        remainings: &[u8],
        remaining_absolute_start: usize
    ) -> Option<(u16, u16)> {
        if remainings.len() < (MIN_LENGTH as usize) {
            return None;
        }

        let mut distance: u16 = 0;
        let mut length: u16 = 0;
        let max_length = remainings.len().min(MAX_LENGTH as usize);

        let mut is_matched_bytes = false;
        if
            let Some(candidates) = self.hashes_map.get(
                &Self::hash(&remainings[0..MIN_LENGTH as usize])
            )
        {
            for candidate in candidates.iter().rev() {
                let mut current_length: u16 = MIN_LENGTH;
                let new_distance: u16 = (remaining_absolute_start - candidate) as u16;

                loop {
                    if (current_length as usize) < max_length {
                        let source_idx =
                            candidate + ((current_length as usize) % (new_distance as usize));

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
                    is_matched_bytes = true;
                    distance = new_distance;
                    length = current_length;

                    if (current_length as usize) == max_length {
                        return Some((distance, length));
                    }
                }
            }
        }
        if !is_matched_bytes {
            if let Some(candidates) = self.hashes_map.get(&Self::hash(&remainings[0..1])) {
                for candidate in candidates.iter().rev() {
                    let mut current_length: u16 = 0;
                    let new_distance: u16 = (remaining_absolute_start - candidate) as u16;

                    loop {
                        if (current_length as usize) >= max_length {
                            break;
                        }

                        let source_idx =
                            candidate + ((current_length as usize) % (new_distance as usize));

                        if remainings[current_length as usize] != original[source_idx] {
                            break;
                        }

                        current_length += 1;
                    }

                    if current_length > length {
                        distance = new_distance;
                        length = current_length;

                        if (current_length as usize) == max_length {
                            return Some((distance, length));
                        }
                    }
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
