//! Segment tree
//! Attention: Have Not Accepted
//!

struct RangeModule {
    head: Node,
}

#[allow(unused)]
impl RangeModule {
    fn new() -> Self {
        Self {
            head: Node {
                l: None,
                r: None,
                tracked: false,
            },
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        self.head.add_range(left, right, 1, 1_000_000_000)
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self.head.query_range(left, right, 1, 1_000_000_000)
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        self.head.remove_range(left, right, 1, 1_000_000_000)
    }
}

#[derive(Eq, PartialEq)]
struct Node {
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
    tracked: bool,
}

#[allow(unused)]
impl Node {
    /// pl <= l < r <= pr
    fn add_range(&mut self, l: i32, r: i32, pl: i32, pr: i32) {
        if l == r {
            return;
        }
        if l <= pl && pr <= r {
            self.tracked = true;
            return;
        }
        // 1. l < mid < r -> all;
        // 2. mid <= l < r -> right;
        // 3. l < r < mid -> left
        let mid = (pl + pr) >> 1;
        if l < mid {
            if self.l == None {
                self.l = Some(Box::new(Node {
                    l: None,
                    r: None,
                    tracked: false,
                }));
            }
            self.l.as_mut().unwrap().add_range(l, mid.min(r), pl, mid);
        }
        if mid < r {
            if self.r == None {
                self.r = Some(Box::new(Node {
                    l: None,
                    r: None,
                    tracked: false,
                }));
            }
            self.r.as_mut().unwrap().add_range(l.max(mid), r, mid, pr);
        }
    }

    fn query_range(&self, l: i32, r: i32, pl: i32, pr: i32) -> bool {
        if self.tracked == true {
            return true;
        }
        if l <= pl && pr <= r {
            return self.tracked;
        }
        let mid = (pl + pr) >> 1; // mid = 16
        let mut ans1 = true;
        let mut ans2 = true;
        if l < mid {
            ans1 = false;
            if let Some(v) = self.l.as_ref() {
                ans1 = v.query_range(l, mid.min(r), pl, mid);
            }
        }
        if mid < r {
            ans2 = false;
            if let Some(v) = self.r.as_ref() {
                ans2 = v.query_range(l.max(mid), r, mid, pr);
            }
        }
        ans1 && ans2
    }

    fn remove_range(&mut self, l: i32, r: i32, pl: i32, pr: i32) {
        if self.tracked {
            self.add_range(pl, l, pl, pr);
            self.add_range(r, pr, pl, pr);
        }
        if l <= pl && pr <= r {
            self.tracked = false;
            return;
        }
        self.tracked = false;
        let mid = (pl + pr) >> 1;
        if l < mid {
            if let Some(v) = self.l.as_mut() {
                v.remove_range(l, mid.min(r), pl, mid);
                if l <= pl && mid <= mid.min(r) {
                    self.l.take();
                }
            }
        }
        if mid < r {
            if let Some(v) = self.r.as_mut() {
                v.remove_range(l.max(mid), r, mid, pr);
                if l.max(mid) <= mid && pr <= r {
                    self.r.take();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::p715::RangeModule;

    #[test]
    fn test1() {
        let mut obj = RangeModule::new();
        obj.add_range(10, 20);
        obj.remove_range(14, 16);
        assert_eq!(obj.query_range(10, 14), true, "eq 1");
        assert_eq!(obj.query_range(13, 15), false, "eq 2");
        assert_eq!(obj.query_range(16, 17), true, "eq 3");
    }

    #[test]
    fn test2() {
        let mut obj = RangeModule::new();
        obj.add_range(5, 7);
        assert_eq!(obj.query_range(2, 7), false, "eq 1");
        obj.add_range(6, 9);
        assert_eq!(obj.query_range(2, 9), false, "eq 2");
        obj.add_range(2, 7);
        obj.remove_range(3, 10);
        obj.remove_range(1, 8);
        obj.remove_range(1, 10);
        assert_eq!(obj.query_range(4, 7), false, "eq 3");
    }
}
