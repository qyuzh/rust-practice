//! # Problem Description
//!
//! Given an array `nums` and a positive integer `k`, find the k-th largest
//! subsequence sum.
//!
//! ## Constraints:
//! - -10^9 <= `nums[i]` <= 10^9
//!
//! # Solution
//!
//! Consider `[2, 4, -1]`
//!
//! let `sum_positive = 2 + 4 = 6`
//!
//! 1-th: `6 = sum_positive` \
//! 2-th: `5 = 2 + 6 - 1 = sum_positive - |-1|` \
//! 3-th: `4 = 4         = sum_positive - |2|` \
//! 4-th: `3 = 4 - 1     = sum_positive - (|2| + |-1|)` \
//! 5-th: `2 = 2         = sum_positive - (|2| + |4|)` \
//! 6-th: `1 = 2 - 1     = sum_positive - (|4| + |-1|)` \
//! 7-th: `0 = 0         = sum_positive - sum_positive` \
//! 8-th: `-1 = -1       = sum_positive - (|4| + |2| + |-1|)` \
//!
//! let `nums_abs = abs(nums)`
//!
//! Origin problem
//! => find the k-th smallest subsequence(includes empty) sum in `nums_abs` (P1)
//!
//! let `k_smallest_seq_sum` is the answer of P1
//!
//! Origin problem's answer = `sum_positive - k_smallest_seq_sum`
//!

use std::cmp::Reverse;

/// Run in O(nlogn + klogU), explain as follows.
pub fn k_sum(mut nums: Vec<i32>, k: i32) -> i64 {
    let mut sum = sum_of_positive(&nums);

    abs(&mut nums);

    sum - k_smallest_seq_sum(nums, k)
}

pub fn k_sum2(mut nums: Vec<i32>, k: i32) -> i64 {
    let mut sum = sum_of_positive(&nums);

    abs(&mut nums);

    sum - k_smallest_seq_sum2(nums, k)
}

/// Run in O(nlogn + klogk)/O(k)
fn k_smallest_seq_sum2(mut nums: Vec<i32>, mut k: i32) -> i64 {
    if nums.is_empty() || k == 1 {
        return 0;
    }

    nums.sort_unstable();

    let mut pq = std::collections::BinaryHeap::new();
    pq.push(Reverse((nums[0] as i64, 1)));
    while k > 2 {
        let Reverse((s, i)) = pq.pop().unwrap(); // SAFETY: obvious

        if i < nums.len() {
            pq.push(Reverse((s + nums[i] as i64, i + 1)));
            pq.push(Reverse((s + nums[i] as i64 - nums[i - 1] as i64, i + 1)));
        }

        k -= 1;
    }

    pq.pop().unwrap().0 .0 // SAFETY: there must are an element in it
}

/// Run in O(nlogn + klogU)/O(min(k, n)) in which n = nums.len(), U = sum{nums}
fn k_smallest_seq_sum(mut nums: Vec<i32>, k: i32) -> i64 {
    nums.sort_unstable();

    let mut l: i64 = 0; // 空序列时为0
    let mut r: i64 = nums.iter().map(|&x| x as i64).sum();

    while l < r {
        let mid = (l + r) >> 1;

        let mut cnt = k - 1; // 减去空子序列

        dfs(0, mid, &nums, &mut cnt);

        if cnt == 0 {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    l
}

/// 判断`nums[i..]`中是否有cnt个子序列的和 <= s
///
/// Run in O(cnt)
///
/// # Args:
/// - nums that `nums[i]` <= `nums[i + 1]`
///
/// # Outputs
/// - cnt == 0 -> 有
/// - cnt != 0 -> 没有
fn dfs(i: usize, s: i64, nums: &Vec<i32>, cnt: &mut i32) {
    if *cnt == 0 || i == nums.len() || s < nums[i] as i64 {
        return;
    }

    *cnt -= 1; // 执行到这里, 意味着, 找到一个子序列, 其和 <= s

    dfs(i + 1, s - nums[i] as i64, nums, cnt); // 选择 i
    dfs(i + 1, s, nums, cnt); // 不选 i
}

#[inline]
fn abs(nums: &mut [i32]) {
    for x in nums.iter_mut() {
        if *x < 0 {
            *x = -*x;
        }
    }
}

#[inline]
fn sum_of_positive(nums: &[i32]) -> i64 {
    let mut sum = 0;
    for &x in nums.iter() {
        if x > 0 {
            sum += x as i64;
        }
    }
    sum
}
