/// Why can't do it right in one go?
///
/// taxonomy: the position of ?
///
/// s = reg"(0[1-9]|1[0-1]):[0-5][0-9]"
pub fn find_latest_time(mut s: String) -> String {
    // SAFETY: s only consists of ascii charactar
    let mut bytes = unsafe { s.as_bytes_mut() };

    if bytes[0] == b'?' {
        if bytes[1] == b'?' {
            // "??"
            bytes[0] = b'1';
            bytes[1] = b'1';
        } else if bytes[1] == b'1' || bytes[1] == b'0' {
            // "?(0|1)"
            bytes[0] = b'1';
        } else {
            // "?[2-9]"
            bytes[0] = b'0';
        }
    } else if bytes[1] == b'?' {
        if bytes[0] == b'0' {
            // "0?"
            bytes[1] = b'9';
        } else {
            // "1?"
            bytes[1] = b'1';
        }
    }

    if bytes[3] == b'?' {
        bytes[3] = b'5';
    }

    if bytes[4] == b'?' {
        bytes[4] = b'9';
    }

    s
}

/// Why can't do it right in one go?
/// find the maximum distance NOT the difference
pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    use std::sync::OnceLock;

    static PRIMES: OnceLock<HashSet<i32>> = OnceLock::new();
    let primes = PRIMES.get_or_init(|| {
        [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ]
        .into()
    });

    let mut left = 0;
    let mut right = 0;

    for (i, x) in nums.iter().enumerate() {
        if primes.contains(x) {
            left = i;
            break;
        }
    }

    for (i, x) in nums.iter().enumerate().rev() {
        if primes.contains(x) {
            right = i;
            break;
        }
    }

    (right - left) as i32
}

/// How to solve this?
///
/// ```
/// 2: 2 4 6 8  10 12 14   16     ...30
/// 3: 3   6  9    12   15        ...30
/// 5: 5        10      15    20  ...30
/// ```
///
/// runs in O(n*2^nlog(k * min(coins)))/O(1)
pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
    let check = |m: i64| -> bool {
        let mut cnt = 0i64;
        for mask in (1..(1usize << coins.len())) {
            let mut lcm_res = 1i64;
            for (j, &coin) in coins.iter().enumerate() {
                if (mask >> j) & 1 == 1 {
                    lcm_res = lcm(lcm_res, coin as i64);
                    if lcm_res > m {
                        break;
                    }
                }
            }

            cnt += m / lcm_res * if mask.count_ones() % 2 == 1 { 1 } else { -1 };
        }

        cnt >= k as i64
    };

    let mut l = k as i64;
    let mut r = *coins.iter().min().unwrap() as i64 * k as i64;

    while l < r {
        let mid = (l + r) >> 1;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    r
}

fn lcm<T>(mut a: T, mut b: T) -> T
where
    T: Eq
        + Ord
        + Default
        + Copy
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    a * b / gcd(a, b)
}

fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Eq + Default + Copy + std::ops::Rem<Output = T>,
{
    while b != T::default() {
        (a, b) = (b, a % b)
    }
    a
}

use std::collections::HashMap;
const INF: i32 = i32::MAX >> 2;

/// runs in O(nmlogU)/O(nmlogU), in which U = max(nums)
fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
    let mut memo = HashMap::new();
    let ans = dfs(0, 0, -1, &mut memo, &nums, &and_values);

    if ans < INF {
        ans
    } else {
        -1
    }
}

fn dfs(
    i: usize,
    j: usize,
    and: i32,
    memo: &mut HashMap<(usize, usize, i32), i32>,
    nums: &Vec<i32>,
    and_values: &Vec<i32>,
) -> i32 {
    let n = nums.len();
    let m = and_values.len();

    // prune
    if m - j > n - i {
        return INF;
    }

    // reach end
    if j == m {
        return if i == n { 0 } else { INF };
    }

    // memo optimization
    if memo.contains_key(&(i, j, and)) {
        return *memo.get(&(i, j, and)).unwrap();
    }

    let and = and & nums[i];

    // and is monotone descrasing
    if and < and_values[j] {
        return INF;
    }

    let mut res = dfs(i + 1, j, and, memo, nums, and_values); // not divide

    // divide
    if and == and_values[j] {
        res = res.min(dfs(i + 1, j + 1, -1, memo, nums, and_values) + nums[i]);
    }

    memo.insert((i, j, and), res);

    res
}

#[cfg(test)]
mod test {
    use crate::wc393::{find_latest_time, gcd, lcm};

    #[test]
    fn test_find_latest_time() {
        assert_eq!(find_latest_time("0?:00".into()), "09:00");
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(100i8, 20i8), 20i8);
        assert_eq!(gcd(100i16, 20i16), 20i16);
        assert_eq!(gcd(100i32, 20i32), 20i32);
        assert_eq!(gcd(100i64, 20i64), 20i64);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(2, 5), 10);
        assert_eq!(lcm(15, 6), 30)
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn test_bit_ops() {
        assert_eq!((-1) & 2, 2);
    }
}
