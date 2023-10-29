/// A
///
pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    for i in 0..32 {
        let mut cnt = 0;
        for &x in nums.iter() {
            if ((x >> i) & 1) == 1 {
                cnt += 1;
            }
        }
        if cnt >= k {
            ans |= 1 << i;
        }
    }
    ans
}

/// B
///
pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let mut sum1 = 0i64;
    let mut sum2 = 0i64;
    let mut has_zero1 = false;
    let mut has_zero2 = false;
    for &x in nums1.iter() {
        if x == 0 {
            has_zero1 = true;
            sum1 += 1;
        }
        sum1 += x as i64;
    }
    for &x in nums2.iter() {
        if x == 0 {
            has_zero2 = true;
            sum2 += 1;
        }
        sum2 += x as i64;
    }

    if sum1 == sum2 {
        sum1
    } else if sum1 < sum2 {
        if has_zero1 {
            sum2
        } else {
            -1
        }
    } else {
        if has_zero2 {
            sum1
        } else {
            -1
        }
    }
}

/// C
pub fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as i64;
    let mut f = vec![0i64; nums.len()];
    f[2] = (k - *nums.iter().take(3).max().unwrap() as i64).max(0);
    for i in 3..nums.len() {
        if nums[i - 1] as i64 >= k || nums[i - 2] as i64 >= k {
            f[i] = f[i - 1].min(f[i - 2]);
        } else {
            f[i] = f[i - 1] + (k - nums[i] as i64).max(0);
            f[i] = f[i].min(f[i - 2] + (k - nums[i - 1] as i64).max(0));
            f[i] = f[i].min(f[i - 3] + (k - nums[i - 2] as i64).max(0));
        }
    }
    f[nums.len() - 1]
}

const MAX_BITS: usize = 16;

/// D
///
pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
    let mut g = vec![vec![]; coins.len()];
    for e in edges.iter() {
        g[e[0] as usize].push(e[1] as usize);
        g[e[1] as usize].push(e[0] as usize);
    }
    let mut f = vec![vec![i32::MIN; MAX_BITS]; coins.len()];
    let ans = dfs(&coins, &g, &mut f, 0, 0, k, 0);
    ans
}

fn dfs(
    coins: &Vec<i32>,
    g: &Vec<Vec<usize>>,
    f: &mut Vec<Vec<i32>>,
    r: usize,
    t: usize,
    k: i32,
    p: usize,
) -> i32 {
    if t >= MAX_BITS {
        return 0;
    }
    if f[r][t] != i32::MIN {
        return f[r][t];
    }
    let mut ans1 = (coins[r] >> t) - k; // Needs to shift right t bits
    let mut ans2 = coins[r] >> (t + 1);
    for &nxt in g[r].iter() {
        if nxt == p {
            continue;
        }
        ans1 += dfs(coins, g, f, nxt, t, k, r);
        ans2 += dfs(coins, g, f, nxt, t + 1, k, r);
    }
    f[r][t] = ans1.max(ans2);
    f[r][t]
}

#[cfg(test)]
mod test {
    use crate::wc_369::{maximum_points, min_increment_operations};

    #[test]
    fn test_a() {}

    #[test]
    fn test_b() {}

    #[test]
    fn test_c() {
        //                                             0, 0, 8, 8, 8,10,14,14
        assert_eq!(
            min_increment_operations(vec![2, 1, 1, 7, 2, 3, 5, 6], 9),
            13
        );
    }

    #[test]
    fn test_d() {
        assert_eq!(
            maximum_points(
                vec![vec![1, 0], vec![2, 1], vec![3, 1]],
                vec![8, 2, 7, 1],
                2
            ),
            11
        )
    }
}
