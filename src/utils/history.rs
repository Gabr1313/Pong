pub struct History<T: Copy> {
    v: Vec<T>,
    index: usize,
}

impl<T: Copy> History<T> {
    pub fn new(capacity: usize, elem: T) -> Self {
        assert!(capacity > 0);
        History {
            v: vec![elem; capacity],
            index: capacity,
        }
    }
    pub fn push(&mut self, elem: T) {
        self.index = (self.index + 1) % self.v.len();
        self.v[self.index] = elem;
    }
    pub fn get_value(&self, index: usize) -> Option<T> {
        if index >= self.v.len() {
            return None;
        }
        Some(self.v[(self.index + (self.v.len() - index)) % self.v.len()])
    }
    pub fn first(&self) -> T {
        self.v[self.index]
    }
}
