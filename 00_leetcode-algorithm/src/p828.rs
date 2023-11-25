/// Count Unique Characters of All Substrings of a Given String
///
/// Get answer in O(n)/O(n)
///
/// Case (consider A in position 4, X is not A)
/// A X X X A X X X A
/// 0 1 2 3 4 5 6 7 8
/// = (4 - 0) * (8 - 4) = 16
///
pub fn unique_letter_string(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut right = vec![0; bytes.len()];
    let mut record = [bytes.len() as i32; 26];
    for i in (0..bytes.len()).rev() {
        let idx = (bytes[i] - b'A') as usize;
        right[i] = record[idx];
        record[idx] = i as i32;
    }

    let mut ans = 0;

    record.fill(-1);
    for i in 0..bytes.len() {
        let idx = (bytes[i] - b'A') as usize;
        ans += (i as i32 - record[idx]) * (right[i] - i as i32);
        record[idx] = i as i32;
    }

    ans
}
