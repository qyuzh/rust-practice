pub fn score_of_string(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut ans = 0;
    for (&i, &n) in bytes.iter().zip(bytes.iter().skip(1)) {
        ans += (n as i32 - i as i32).abs();
    }
    ans
}
