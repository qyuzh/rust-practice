/// We have some conclusions:
/// 1. There are at most one path that includes value 1
/// 2. The answer of the nodes that are not in the above path is 1
/// 3. The answer of the nodes that are in the above path from bottom to top is not descending.
///
/// Get answer in O(n)/O(n)
pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
    let mut g = vec![vec![]; parents.len()];
    for (i, &x) in parents.iter().enumerate().skip(1) {
        g[x as usize].push(i);
    }
    let mut ans = vec![0; parents.len()];
    let mut p2003 = P2003 {
        g: &g,
        vis: vec![false; *nums.iter().max().unwrap() as usize + 2],
        nums,
        has1: vec![false; parents.len()],
        min: 1,
    };
    p2003.dfs_check_has1(&mut ans, 0);
    p2003.dfs_calculate_has1_path(&mut ans, 0);
    ans
}

struct P2003<'a> {
    g: &'a Vec<Vec<usize>>,
    nums: Vec<i32>,
    has1: Vec<bool>,
    vis: Vec<bool>,
    min: usize,
}

impl P2003<'_> {
    fn dfs_check_has1(&mut self, ans: &mut Vec<i32>, node: usize) {
        if self.nums[node] == 1 {
            self.has1[node] = true;
        }
        for &x in self.g[node].iter() {
            self.dfs_check_has1(ans, x);
            self.has1[node] = self.has1[node] || self.has1[x];
        }
        if !self.has1[node] {
            ans[node] = 1;
        }
    }

    fn dfs_calculate_has1_path(&mut self, ans: &mut Vec<i32>, node: usize) {
        for &x in self.g[node].iter() {
            if self.has1[x] {
                self.dfs_calculate_has1_path(ans, x);
            }
        }
        for &x in self.g[node].iter() {
            if !self.has1[x] {
                self.dfs_calculate_has1_path(ans, x);
            }
        }
        self.vis[self.nums[node] as usize] = true;
        while self.vis[self.min] {
            self.min += 1;
        }
        if self.has1[node] {
            ans[node] = self.min as i32;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::p2003::smallest_missing_value_subtree;

    #[test]
    fn test_v1() {
        assert_eq!(
            smallest_missing_value_subtree(vec![-1, 0, 1, 0, 3, 3], vec![5, 4, 6, 2, 1, 3]),
            [7, 1, 1, 4, 2, 1]
        );
    }
}
