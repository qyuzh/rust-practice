/// find 3 non-overlapping subarrays of length k
///
/// let `f[i][j]` represent the maximum sum of i subarrays
/// in array formed with j numbers on the left.
///
///
/// 1-indexed
pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut sum = vec![0; n + 1];
    for i in 1..=n {
        sum[i] = sum[i - 1] + nums[i - 1] as u64;
    }

    let mut f = vec![vec![0; 4]; n + 2];
    for i in (1..=(n - k + 1)).rev() {
        for j in 1..4 {
            f[i][j] = f[i + 1][j].max(f[i + k][j - 1] + sum[i + k - 1] - sum[i - 1]);
        }
    }

    let mut ans = vec![0; 3];
    let (mut i, mut j, mut idx) = (1, 3, 0);
    while j > 0 {
        if f[i + 1][j] > f[i + k][j - 1] + sum[i + k - 1] - sum[i - 1] {
            i += 1;
        } else {
            ans[idx] = (i - 1) as i32;
            idx += 1;
            i += k;
            j -= 1;
        }
    }

    ans
}
