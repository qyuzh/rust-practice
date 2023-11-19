///
pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut ht = std::collections::HashMap::new();
    for &x in nums.iter() {
        let sod = get_digit_sum(x);
        let val = ht.entry(sod).or_insert(-x - 1);
        ans = ans.max(*val + x);
        if *val < x {
            *val = x;
        }
    }
    ans
}

#[inline]
fn get_digit_sum(mut x: i32) -> i32 {
    let mut ans = 0;
    while x > 0 {
        ans += x % 10;
        x /= 10;
    }
    ans
}
