use super::encode_arena::Node;

#[derive(Debug, Default)]
pub struct MinHeap {
    pub inner: Vec<Node>,
}

impl MinHeap {
    pub fn parent(&self, idx: usize) -> usize {
        return idx.saturating_sub(1) / 2;
    }

    pub fn left(&self, idx: usize) -> usize {
        return 2 * idx + 1;
    }
    pub fn right(&self, idx: usize) -> usize {
        return 2 * idx + 2;
    }

    pub fn insert(&mut self, node: Node) {
        self.inner.push(node);
        self.heapify_up(self.inner.len() - 1)
    }

    pub fn pop_min(&mut self) -> Option<Node> {
        if self.inner.is_empty() {
            return None;
        }

        let len = self.inner.len();
        self.inner.swap(0, len - 1);

        let out = self.inner.pop();

        self.heapify_down(0);

        out
    }

    pub fn heapify_up(&mut self, mut current: usize) {
        while current > 0 {
            let parent = self.parent(current);

            if self.inner[parent] <= self.inner[current] {
                break;
            }

            self.inner.swap(parent, current);
            current = parent;
        }
    }
    pub fn heapify_down(&mut self, mut current: usize) {
        let len = self.inner.len();

        while current < len {
            let left = self.left(current);
            let right = self.right(current);

            let Some(left_node) = self.inner.get(left) else {
                break;
            };

            let change = if let Some(right_node) = self.inner.get(right) {
                if left_node < right_node { left } else { right }
            } else {
                left
            };

            if self.inner[current] <= self.inner[change] {
                break;
            }

            self.inner.swap(current, change);
            current = change;
        }
    }
}
