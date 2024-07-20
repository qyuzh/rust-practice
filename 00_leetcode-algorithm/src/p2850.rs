/// ATTENTION: wrong solution, correct solution refers to cpp.cc#Solution2850
pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    // get_min_distance
    let mut ht = std::collections::HashMap::<(usize, usize), i32>::new();
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] > 1 {
                ht.insert((i, j), grid[i][j] - 1);
            }
        }
    }
    let mut get_min_distance = |i: usize, j: usize| -> i32 {
        let mut min_pos = None;
        let mut min_val = i32::MAX;
        for &(x, y) in ht.keys() {
            let val = (i.max(x) - i.min(x) + j.max(y) - j.min(y)) as i32;
            if val < min_val {
                min_val = val;
                min_pos = Some((x, y));
            }
        }
        if let Some(p) = &min_pos {
            if *ht.get(p).unwrap() == 0 {
                ht.remove(p);
            } else {
                *ht.get_mut(p).unwrap() -= 1;
            }
        }
        min_val
    };

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                let t = get_min_distance(i, j);
                ans += t;
            }
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimum_moves() {
        let t = minimum_moves(vec![vec![3, 2, 0], vec![0, 1, 0], vec![0, 3, 0]]);
    }
}
