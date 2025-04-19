/// Runs in O(nlogn) time and O(logn) space.
pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    nums.sort_unstable();
    let mut ans = 0;
    let n = nums.len();
    for (idx, &num) in nums.iter().enumerate() {
        let l = nums[..idx].partition_point(|&x| x + num < lower);
        let r = nums[..idx].partition_point(|&x| x + num <= upper);
        ans += (r - l) as i64;
    }
    ans
}
