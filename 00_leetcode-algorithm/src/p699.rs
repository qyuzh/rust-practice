/// runs in O(n^2)/O(n)
pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    let n = positions.len();
    let mut hs = vec![0; n];
    for (i, p) in positions.iter().enumerate() {
        let l1 = p[0];
        let r1 = p[0] + p[1];
        hs[i] = p[1];
        for (j, p2) in positions.iter().take(i).enumerate() {
            let l2 = p2[0];
            let r2 = p2[0] + p2[1];
            if !(l1 >= r2 /* i right on j */
                || l2 >= r1/* j right on i */)
            {
                hs[i] = hs[i].max(hs[j] + p[1]);
            }
        }
    }
    for i in 1..n {
        hs[i] = hs[i].max(hs[i - 1]);
    }
    hs
}

/// TODO: segment tree implementation
pub fn falling_squares2(ps: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
