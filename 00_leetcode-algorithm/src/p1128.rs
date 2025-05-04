/// Runs in O(n) time and O(1) space.
pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut count = [0; 100];
    let mut ans = 0;
    for d in dominoes {
        let (a, b) = (d[0], d[1]);
        if a > b {
            let idx = (b * 10 + a) as usize;
            ans += count[idx];
            count[idx] += 1;
        } else {
            let idx = (a * 10 + b) as usize;
            ans += count[idx];
            count[idx] += 1;
        }
    }
    ans
}
