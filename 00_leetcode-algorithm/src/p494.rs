use std::collections::HashMap;

/// runs in O(nm)/O(m), in which n = nums.len(), m = sum{nums}
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut t: HashMap<i32, usize> = HashMap::from_iter([(0, 1)]);
    for (idx, &x) in nums.iter().enumerate() {
        let mut tt = HashMap::new();
        for (&s, &cnt) in t.iter() {
            tt.entry(s + x).and_modify(|x| *x += cnt).or_insert(cnt);
            tt.entry(s - x).and_modify(|x| *x += cnt).or_insert(cnt);
        }
        t = tt;
    }
    t.get(&target).map_or(0, |v| *v as i32)
}
