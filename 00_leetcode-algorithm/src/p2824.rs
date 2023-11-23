/// version 1 in O(n^2)/O(1)
pub fn count_pairs1(nums: Vec<i32>, target: i32) -> i32 {
    let mut ans = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] < target {
                ans += 1;
            }
        }
    }
    ans
}

/// version 2 in O(nlogn)/O(1)
/// We can sort because of a + b = b + a
pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    nums.sort();
    let mut ans = 0;
    let (mut l, mut r) = (0, nums.len() - 1);
    while l < r {
        if nums[l] + nums[r] < target {
            ans += (r - l) as i32;
            l += 1;
        } else {
            r -= 1;
        }
    }
    ans
}
