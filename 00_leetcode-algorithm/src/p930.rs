/// 考虑问题: 以a结尾的和为goal的区间有多少个. all i that sum of [i, a] equals goal
/// runs in O(n)/O(n)
pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let mut ps = vec![0; nums.len() + 1]; // prefix sum array
    nums.iter()
        .enumerate()
        .for_each(|(idx, &v)| ps[idx + 1] = ps[idx] + v);
    let mut ans = 0;
    let mut ht = std::collections::BTreeMap::from_iter([(0, 1)]);
    for &r in ps.iter().skip(1) {
        let l = r - goal;
        ans += *ht.get(&l).unwrap_or(&0);
        ht.entry(r).and_modify(|v| *v += 1).or_insert(1);
    }
    ans
}
