use std::collections::HashMap;

pub fn max_operations(nums: Vec<i32>) -> i32 {
    let mut memo = HashMap::new();
    dfs(&nums, 0, nums.len() - 1, -1, &mut memo) as i32
}

/// dfs with memo
fn dfs(
    nums: &[i32],
    l: usize,
    r: usize,
    target: i32,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if l >= r {
        return 0;
    }

    if let Some(&t) = memo.get(&(l, r)) {
        return t;
    }

    let mut ret = 0;

    // case 1: first two elements
    if nums[l] + nums[l + 1] == target || target == -1 {
        ret = ret.max(dfs(nums, l + 2, r, nums[l] + nums[l + 1], memo) + 1);
    }

    // case 2: last two elements
    if nums[r] + nums[r - 1] == target || target == -1 {
        // avoid to subtract with overflow
        if r >= 2 {
            ret = ret.max(dfs(nums, l, r - 2, nums[r] + nums[r - 1], memo) + 1);
        } else {
            ret = ret.max(1);
        }
    }

    // case 3: one first and one last
    if nums[l] + nums[r] == target || target == -1 {
        ret = ret.max(dfs(nums, l + 1, r - 1, nums[l] + nums[r], memo) + 1);
    }

    memo.insert((l, r), ret);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_operations() {
        let t = max_operations([1, 1, 1, 1, 1, 1].into());
        assert_eq!(t, 3);
    }
}
