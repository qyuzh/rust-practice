/// # 公式推倒
///
/// a b \
/// c n
///
/// c = a ^ c1 \
/// b = a ^ b1 \
/// n = c1 ^ b1 ^ a = b ^ c ^ a
///
/// # Complexity
/// Runs in O(mn*log(mn))
pub fn kth_largest_value(mut matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut ret = Vec::with_capacity(n * m);

    for i in 0..n {
        for j in 0..m {
            let mut ans = matrix[i][j];

            if i > 0 {
                ans ^= matrix[i - 1][j];
            }

            if j > 0 {
                ans ^= matrix[i][j - 1];
            }

            if i > 0 && j > 0 {
                ans ^= matrix[i - 1][j - 1];
            }

            matrix[i][j] = ans;

            ret.push(matrix[i][j]);
        }
    }

    *ret.select_nth_unstable(m * n - k as usize).1
}
