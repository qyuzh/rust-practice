/// Count the number of some bit supposed to be 1 in the final combination.
/// The answer is the maximum count of some bit.
/// Runs in O(n*C) time.
pub fn largest_combination(candidates: Vec<i32>) -> i32 {
    let max = *candidates.iter().max().unwrap();
    let bit_len = 32 - max.leading_zeros() as usize;
    let mut ans = 0;
    for i in 0..bit_len {
        let mut cnt = 0;
        for x in candidates.iter() {
            if x & (1 << i) != 0 {
                cnt += 1;
            }
        }
        ans = ans.max(cnt);
    }
    ans
}
