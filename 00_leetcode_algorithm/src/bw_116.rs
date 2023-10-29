/// A
///
pub fn sum_counts(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            let mut hs = std::collections::HashSet::new();
            for k in i..j + 1 {
                hs.insert(nums[k]);
            }
            ans += (hs.len() * hs.len()) as i32;
        }
    }
    ans
}

/// A v2
pub fn sum_counts_v2(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for i in 0..n {
        let mut hs = std::collections::HashSet::new();
        for j in i..n {
            hs.insert(nums[j]);
            ans += (hs.len() * hs.len()) as i32;
        }
    }
    ans
}

/// B
///
pub fn min_changes(s: String) -> i32 {
    let mut ans = 0;
    let mut ones = 0;
    let mut zeros = 0;
    for &c in s.as_bytes() {
        if c == b'0' {
            if ones > 0 {
                if ones % 2 != 0 {
                    ans += 1;
                    zeros -= 1;
                }
                ones = 0;
            }
            zeros += 1;
        } else {
            if zeros > 0 {
                if zeros % 2 != 0 {
                    ans += 1;
                    ones -= 1;
                }
                zeros = 0;
            }
            ones += 1;
        }
    }
    ans
}

/// B v2
/// Consider the final result sequence, it must have the pattern
/// `aa|bb|aa|bb...aa` in which `a = 0 or 1`, `b = 0 or 1`.
pub fn min_changes_v2(s: String) -> i32 {
    let mut ans = 0;
    for i in (1..s.len()).step_by(2) {
        if s.as_bytes()[i - 1] != s.as_bytes()[i] {
            ans += 1;
        }
    }
    ans
}

/// C
/// deleting some or no elements
/// `f[t][i] = f[t - nums[i][i - 1]`
pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
    let mut f = vec![i32::MIN; target as usize + 1];
    f[0] = 0;
    for i in 0..nums.len() {
        for t in (nums[i]..target + 1).rev() {
            f[t as usize] = f[t as usize].max(f[(t - nums[i]) as usize] + 1);
        }
    }
    if f[target as usize] < 0 { -1 } else { f[target as usize] }
}

/// D
/// Segment tree
pub fn sum_counts_d(_nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::bw_116::length_of_longest_subsequence;

    #[test]
    fn check_c() {
        assert_eq!(length_of_longest_subsequence(vec![1, 2, 3, 4, 5], 9), 3);
        assert_eq!(length_of_longest_subsequence(vec![4, 1, 3, 2, 1, 5], 7), 4);
        assert_eq!(length_of_longest_subsequence(vec![1, 1, 5, 4, 5], 3), -1);
        assert_eq!(length_of_longest_subsequence(vec![1, 1], 2), 2);
        assert_eq!(length_of_longest_subsequence(vec![1000], 1000), 1);
        assert_eq!(length_of_longest_subsequence(vec![2, 3], 3), 1);
        assert_eq!(length_of_longest_subsequence(vec![3, 5, 2, 3, 4], 12), 4);
    }
}
