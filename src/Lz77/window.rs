use std::collections::VecDeque;

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
}
