pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    let mut ans = 0i64;
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < j {
        let mut x = nums[i]; // 56
        let mut y = nums[j]; // 123
        while y != 0 {
            x *= 10;
            y /= 10;
        } // end of this line: x = 56000, y = 0
        ans += (x + nums[j]) as i64;
        i += 1;
        j -= 1;
    }
    if i == j {
        ans += nums[i] as i64;
    }
    ans
}
