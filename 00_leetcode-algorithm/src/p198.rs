pub fn rob(nums: Vec<i32>) -> i32 {
    // dfs(i) 表示从 nums[0] 到 nums[i] 最多能偷多少
    fn dfs(i: Option<usize>, nums: &[i32], memo: &mut [i32]) -> i32 {
        let Some(i) = i else {
            return 0;
        };

        if memo[i] != -1 {
            return memo[i]; // 之前计算过
        }

        memo[i] =
            dfs(i.checked_sub(1), nums, memo).max(dfs(i.checked_sub(2), nums, memo) + nums[i]);

        memo[i]
    }

    let n = nums.len();
    let mut memo = vec![-1; n]; // -1 表示没有计算过
    dfs(Some(n - 1), &nums, &mut memo) // 从最后一个房子开始思考
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(rob(nums), 4);
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(rob(nums), 12);
    }
}
