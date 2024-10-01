struct BookMyShow {
    rows: Vec<usize>,
    n: usize,
    m: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BookMyShow {
    /// `1 < n < 5e4`
    /// `1 < m < 1e9`
    fn new(n: i32, m: i32) -> Self {
        Self {
            rows: vec![0; n as usize],
            n: n as usize,
            m: m as usize,
        }
    }

    /// runs in O(log(max_row))/O(1)
    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let k = k as usize;
        let max_row = max_row as usize;
        let p = self.find_position(max_row, k);
        if p > max_row {
            return vec![];
        }
        let v = self.rows[p];
        self.rows[p] += k;
        vec![p as i32, v as i32]
    }

    /// runs in O(max_row)/O(1)
    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        let mut k = k as usize;
        let max_row = max_row as usize;
        let p = self.find_position(max_row, 1);
        if p > max_row {
            return false;
        }
        let mut s = 0;
        for &x in self.rows[p..=max_row].iter() {
            s += self.m - x;
            if s >= k {
                break;
            }
        }
        let res = s >= k;
        if res {
            for v in self.rows[p..=max_row].iter_mut() {
                if k > 0 {
                    let r = (*v + k).min(self.m);
                    k -= r - *v;
                    *v = r;
                } else {
                    break;
                }
            }
        }
        res
    }

    /// runs in log(max_row)/O(1)
    fn find_position(&self, max_row: usize, left: usize) -> usize {
        let mut l = 0;
        let mut r = max_row;
        while l < r {
            let mid = (l + r) >> 1;
            if self.m - self.rows[mid] >= left {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        if self.m - self.rows[l] >= left {
            l
        } else {
            l + 1
        }
    }
}
