//! P901, Online Stock Span, Leetcode
//! mono-stack

pub struct StockSpanner {
    mono_stack: Vec<(i32, usize)>,
    idx: usize,
}

impl StockSpanner {
    pub fn new() -> Self {
        let mono_stack = vec![(i32::MAX, 0)];
        Self { mono_stack, idx: 0 }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut ans = 0;
        let mut pre_idx = self.idx;
        loop {
            let (p, idx) = *self.mono_stack.last().unwrap();
            if p <= price {
                self.mono_stack.pop();
                pre_idx = idx;
            } else {
                ans = self.idx - pre_idx + 1;
                self.mono_stack.push((price, pre_idx));
                break;
            }
        }
        self.idx += 1;
        ans as i32
    }
}

#[cfg(test)]
mod test {
    use crate::p901::StockSpanner;

    #[test]
    fn should_work() {
        let mut stock_spanner = StockSpanner::new();
        assert_eq!(stock_spanner.next(100), 1);
        assert_eq!(stock_spanner.next(80), 1);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(70), 2);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(75), 4);
        assert_eq!(stock_spanner.next(85), 6);
    }
}
