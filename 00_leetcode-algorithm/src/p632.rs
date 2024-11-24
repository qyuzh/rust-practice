/// Runs in O(nlogn) time.
pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    // 1. sort
    let mut ordered = vec![];
    for (i, num) in nums.iter().enumerate() {
        for n in num {
            ordered.push((n, i));
        }
    }
    ordered.sort_unstable();

    let mut ans = vec![];
    // 2. sliding window
    let mut l = 0;
    let mut cnts = std::collections::HashMap::new(); // count of group number
    for (r, rv) in &ordered {
        cnts.entry(rv).and_modify(|e| *e += 1).or_insert(1);
        if cnts.len() == nums.len() {
            while let Some(t) = cnts.get_mut(&ordered[l].1) {
                if *t > 1 {
                    *t -= 1;
                    l += 1;
                } else {
                    break;
                }
            }
            if ans.is_empty() || ans[1] - ans[0] > *r - ordered[l].0 {
                ans = vec![ordered[l].0, *r];
            }
        }
    }

    ans.into_iter().copied().collect()
}
