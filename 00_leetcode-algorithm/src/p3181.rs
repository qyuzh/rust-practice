/// p3181, leetcode - Maximum Total Reward using Operations II
///
/// # Solutions
/// 1. sort reward_values, choose from left to right
/// 2. let f[i][j] represensts if it is possible to get j reward using first i rewards
///    `f[i][j] = f[i-1][j] || f[i-1][j-reward_values[i]]`,
///    in which `j - reward_values[i] >= 0` and `j - reward_values[i] < reward_values[i]`
/// 3. what is the maximum reward we can get? `max_reward = max(max_reward) * 2 - 1`.
///    why? consider the last-2 operations.
///
/// # Complexity
/// runs in O(n * max(reward_values)) time and O(max(reward_values)) space.
pub fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
    reward_values.sort();
    let n = reward_values.len();
    const N: usize = 5 * 1e4 as usize;

    let mut f = vec![false; 2 * N];
    f[0] = true;
    for &x in reward_values.iter() {
        let x = x as usize;
        for k in (x..=(2 * x - 1)).rev() {
            if f[k - x] {
                f[k] = true;
            }
        }
    }

    for i in (0..(f.len())).rev() {
        if f[i] {
            return i as i32;
        }
    }

    0
}
