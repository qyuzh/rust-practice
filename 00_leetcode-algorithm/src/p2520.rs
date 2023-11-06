pub fn count_digits(num: i32) -> i32 {
    let mut ans = 0;
    let mut t = num;
    while t > 0 {
        if num % (t % 10) == 0 {
            ans += 1;
        }
        t /= 10;
    }
    ans
}
