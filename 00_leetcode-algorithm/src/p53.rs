pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut f = 0;
    let mut ans = i32::MIN; // ans cannot be zero, consider case [-1]
    for &x in nums.iter() {
        f = x.max(f + x);
        ans = ans.max(f);
    }
    ans
}
