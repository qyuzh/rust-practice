/// n dice that has k faces, k^n ways in total
/// `f[t][[i] = sum_(j_i){f[t - g[j_i]][i - 1]}`
pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    let mut f = vec![0; target as usize + 1];
    // 1. init f, after this, f = 0 1 1 ... 1(k times) 0 0
    for t in 1..k.min(target) as usize + 1 {
        f[t] = 1;
    }
    // 2. iter n - 1 times
    for i in 1..n as usize {
        for t in (i + 1..target as usize + 1).rev() {
            f[t] = 0;
            for j in 1..k as usize + 1 {
                if t >= j && f[t - j] > 0 {
                    f[t] = (f[t] + f[t - j]) % (1e9 as i32 + 7);
                }
            }
        }
        // set f[0..i] = 0
        for t in 0..i + 1 {
            f[t] = 0;
        }
    }
    f[target as usize]
}
