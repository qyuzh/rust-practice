pub struct SmallestInfiniteSet {
    set: std::collections::BTreeSet<i32>,
    min: i32,
}

impl Default for SmallestInfiniteSet {
    fn default() -> Self {
        Self::new()
    }
}

impl SmallestInfiniteSet {
    pub fn new() -> Self {
        Self {
            set: (1..=1000).collect(),
            min: 1001,
        }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        if self.set.is_empty() {
            let v = self.min;
            self.min += 1;
            v
        } else {
            let v = *self.set.iter().next().unwrap();
            self.set.remove(&v);
            v
        }
    }

    pub fn add_back(&mut self, num: i32) {
        self.set.insert(num);
    }
}
