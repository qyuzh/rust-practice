/// # Analysis
/// let `dp[i]` represents the maximum number of dollars that the taxi ends up on label i.
/// `dp[i] = max{dp[i-1], rides[j][1] - rides[j][0] + rides[j][2]}`, in which `rides[j][1] == i`.
///
/// # Complexity
/// get answer in O(k + m + mlogm)/O(k + m), in which `m = rides.len()`, `k = max{rides[j][1]}`
pub fn max_taxi_earnings(_n: i32, rides: Vec<Vec<i32>>) -> i64 {
    let mut ids: Vec<_> = (0..rides.len()).collect();
    ids.sort_unstable_by_key(|x| rides[*x][1]);

    let n = rides[ids[ids.len() - 1]][1] as usize;
    let mut dp = vec![0; n + 1];
    let mut i = 1;
    let mut j = 0;
    while j < rides.len() {
        while i < rides[ids[j]][1] as usize {
            dp[i] = dp[i - 1];
            i += 1;
        }

        dp[i] = dp[i - 1];
        while j < rides.len() && i == rides[ids[j]][1] as usize {
            dp[i] = dp[i].max(
                dp[rides[ids[j]][0] as usize]
                    + (rides[ids[j]][1] - rides[ids[j]][0] + rides[ids[j]][2]) as i64,
            );
            j += 1;
        }

        i += 1;
    }

    dp[n]
}

/// get answer in O(n + m)/O(n + m)
pub fn max_taxi_earnings2(n: i32, rides: Vec<Vec<i32>>) -> i64 {
    let mut groups = vec![vec![]; n as usize + 1];
    for r in rides.iter() {
        groups[r[1] as usize].push((r[0] as usize, (r[1] - r[0] + r[2]) as i64));
    }

    let mut f = vec![0; n as usize + 1];
    for i in 2..=n as usize {
        f[i] = f[i - 1];
        for &(s, t) in groups[i].iter() {
            f[i] = f[i].max(f[s] + t);
        }
    }

    f[n as usize]
}
