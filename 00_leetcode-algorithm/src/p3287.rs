use std::collections::HashSet;

pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
    fn find_ors(nums: impl Iterator<Item = i32>, k: usize) -> Vec<HashSet<i32>> {
        let mut dp = vec![];
        let mut prev = vec![HashSet::new(); k + 1]; // prev[i] := a set with i elements
        prev[0].insert(0); // prev[i] as guard helper
        for (i, y) in nums.enumerate() {
            let range = (0..=std::cmp::min(k - 1, i + 1)).rev();
            for j in range {
                let (before, after) = prev.split_at_mut(j + 1);
                for &x in before[j].iter() {
                    after[0].insert(x | y);
                }
            }
            dp.push(prev[k].clone());
        }
        dp
    }

    let k = k as usize;
    let a = find_ors(nums.iter().copied(), k);
    let b = find_ors(nums.iter().rev().copied(), k);
    let mut max = 0;
    let range = (k - 1)..(nums.len() - k);
    for i in range {
        for &va in a[i].iter() {
            for &vb in b[nums.len() - i - 2].iter() {
                max = max.max(va ^ vb);
            }
        }
    }

    max
}
