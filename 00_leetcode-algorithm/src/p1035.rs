/// runs in O(mn)
pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let m = nums1.len();
    let n = nums2.len();

    let mut f = vec![vec![0; n + 1]; m + 1];
    for x in 1..=m {
        for y in 1..=n {
            f[x][y] = f[x - 1][y].max(f[x][y - 1]);
            if nums1[x - 1] == nums2[y - 1] {
                f[x][y] = f[x][y].max(f[x - 1][y - 1] + 1);
            }
        }
    }

    f[m][n]
}
