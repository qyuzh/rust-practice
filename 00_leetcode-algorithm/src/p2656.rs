pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    let &max = nums.iter().max().unwrap();
    (max + max + k - 1) * k / 2
}
