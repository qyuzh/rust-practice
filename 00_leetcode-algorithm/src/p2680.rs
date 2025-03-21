/// Runs in O(n) time and O(1) space.
pub fn maximum_or(mut nums: Vec<i32>, k: i32) -> i64 {
    let mut all_or = 0;
    let mut fixed = 0;
    for &x in nums.iter() {
        fixed |= all_or & x;
        all_or |= x;
    }
    nums.into_iter()
        .map(|x| (all_or ^ x) as i64 | fixed as i64 | ((x as i64) << k))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_or() {
        assert_eq!(maximum_or(vec![8, 1, 2], 2), 35);
        assert_eq!(maximum_or(vec![10, 8, 4], 1), 30);
        assert_eq!(maximum_or(vec![24, 29, 26], 1), 63);
    }
}
