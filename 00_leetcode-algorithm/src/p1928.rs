/// dp runs in `O(max_time * edges.len())/O(max_time * edges.len())`
///
/// `f[t][j]` represents the min-sum-fees to reach i-th city using exact t-minutes
pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
    let n = passing_fees.len();
    const INF: i32 = i32::MAX / 2;
    let mut f = vec![vec![INF; n]; max_time as usize + 1];
    f[0][0] = passing_fees[0];
    for t in 1..=max_time as usize {
        for edge in edges.iter() {
            let (i, j, c) = (edge[0] as usize, edge[1] as usize, edge[2] as usize);
            if c <= t {
                f[t][i] = f[t][i].min(f[t - c][j] + passing_fees[i]);
                f[t][j] = f[t][j].min(f[t - c][i] + passing_fees[j]);
            }
        }
    }
    let mut ans = INF;
    for ff in f.iter().skip(1) {
        ans = ans.min(ff[n - 1]);
    }
    if ans == INF {
        -1
    } else {
        ans
    }
}
