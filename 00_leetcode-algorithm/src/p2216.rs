/// Greedy in O(n)/O(n)
pub fn min_deletion1(nums: Vec<i32>) -> i32 {
    let mut ans = vec![nums[0]];
    for &x in nums.iter().skip(1) {
        if ans.len() % 2 == 1 {
            if *ans.last().unwrap() != x {
                ans.push(x);
            }
        } else {
            ans.push(x);
        }
    }
    if ans.len() % 2 == 0 {
        (nums.len() - ans.len()) as i32
    } else {
        (nums.len() - ans.len()) as i32 + 1
    }
}

/// Greedy in O(n)/O(1)
pub fn min_deletion(nums: Vec<i32>) -> i32 {
    let (mut pre, mut cnt) = (nums[0], 1);
    for &x in nums.iter().skip(1) {
        if cnt % 2 == 1 {
            if pre != x {
                pre = x;
                cnt += 1;
            }
        } else {
            pre = x;
            cnt += 1
        }
    }
    if cnt % 2 == 0 {
        (nums.len() - cnt) as i32
    } else {
        (nums.len() - cnt) as i32 + 1
    }
}
