/// dp runs in O(n^2 * k)/O(nk), tle when n > 5*10e3 and k > 50
pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;

    let mut f = vec![vec![1; k + 1]; n];
    for i in 1..n {
        for j in 0..i {
            for k in 0..=k {
                if nums[i] == nums[j] {
                    f[i][k] = f[i][k].max(f[j][k] + 1);
                } else if k > 0 {
                    f[i][k] = f[i][k].max(f[j][k - 1] + 1);
                }
            }
        }
    }

    f.into_iter().map(|v| v[k]).max().unwrap_or_default()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximum_length() {
        let ans = maximum_length([1, 2, 1, 1, 3].into(), 2);
        assert_eq!(ans, 4);
    }

    #[test]
    fn test_maximum_length_2() {
        let ans = maximum_length([29, 29, 28].into(), 0);
        assert_eq!(ans, 2);
    }
}
