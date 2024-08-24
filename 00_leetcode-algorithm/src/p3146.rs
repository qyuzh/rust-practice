/// runs in O(n + m)/O(C), in which n = s.len, m = t.len, C = 26
pub fn find_permutation_difference(s: String, t: String) -> i32 {
    // 1. compute the index of each char in s
    let mut ht = [0; 26];
    s.as_bytes()
        .iter()
        .enumerate()
        .for_each(|(idx, &v)| ht[(v - b'a') as usize] = idx as i32);

    // 2. sum of abs-diff of each char in t with the same char in s
    t.as_bytes().iter().enumerate().fold(0, |acc, (idx, &v)| {
        acc + (ht[(v - b'a') as usize].abs_diff(idx as i32) as i32)
    })
}
