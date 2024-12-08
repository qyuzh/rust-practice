use std::collections::HashSet;

/// nums is an array
pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len());
    let mut nums: std::collections::BinaryHeap<_> = nums.into_iter().map(|v| -v).collect();
    while !nums.is_empty() {
        let a = -nums.pop().unwrap();
        let b = -nums.pop().unwrap();
        ans.push(b);
        ans.push(a);
    }
    ans
}

/// find the maximum area of *square* field
pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
    const MOD: u64 = 1e9 as u64 + 7;

    if m == n {
        return ((m - 1) as u64 * (n - 1) as u64 % MOD) as i32;
    }

    let t1 = helper(h_fences, m as usize);
    let t2 = helper(v_fences, n as usize);
    let p = t1.intersection(&t2).max();

    if let Some(&p) = p {
        (p * p % MOD) as i32
    } else {
        -1
    }
}

fn helper(mut fences: Vec<i32>, l: usize) -> HashSet<u64> {
    fences.sort_unstable();

    let mut t = HashSet::new();
    for &x in fences.iter() {
        t.insert(x as u64 - 1);
        t.insert(l as u64 - x as u64);
    }

    t.insert(l as u64 - 1); // don't forget this

    for i in 0..fences.len() {
        for j in i + 1..fences.len() {
            t.insert(fences[j] as u64 - fences[i] as u64);
        }
    }

    t
}

pub fn minimum_cost(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    const INF: usize = usize::MAX >> 2;

    let mut dist = vec![vec![INF; 26]; 26];
    for (i, r) in dist.iter_mut().enumerate() {
        r[i] = 0;
    }
    for ((&c1, &c2), &c) in original.iter().zip(changed.iter()).zip(cost.iter()) {
        let (i, j) = (c1 as usize - 'a' as usize, c2 as usize - 'a' as usize);
        dist[i][j] = dist[i][j].min(c as usize);
    }

    // floyd
    for k in 0..26 {
        for i in 0..26 {
            for j in 0..26 {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    let mut ans = 0;
    for (c1, c2) in source.chars().zip(target.chars()) {
        let (i, j) = (c1 as usize - 'a' as usize, c2 as usize - 'a' as usize);
        if dist[i][j] >= INF {
            return -1;
        }
        ans += dist[i][j] as i64;
    }

    ans
}

#[cfg(test)]
mod test {
    #[test]
    fn test_b() {
        let t = super::maximize_square_area(4, 3, [2, 3].into(), [2].into());
        assert_eq!(t, 4);

        let t = super::maximize_square_area(6, 7, [2].into(), [4].into());
        assert_eq!(t, -1);

        let t = super::maximize_square_area(4, 4, [2].into(), [2, 3].into());
        assert_eq!(t, 9);

        let t = super::maximize_square_area(3, 9, [2].into(), [8, 6, 5, 4].into());
        assert_eq!(t, 4);
    }

    #[test]
    fn test_c() {
        let t = super::minimum_cost(
            "abcd".into(),
            "acbe".into(),
            ['a', 'b', 'c', 'c', 'e', 'd'].into(),
            ['b', 'c', 'b', 'e', 'b', 'e'].into(),
            [2, 5, 5, 1, 2, 20].into(),
        );
        assert_eq!(t, 28);
    }
}
