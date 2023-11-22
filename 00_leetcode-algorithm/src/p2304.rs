pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
    let mut f = vec![vec![-1; grid[0].len()]; grid.len()];
    let mut ans = i32::MAX;
    for i in 0..grid[0].len() {
        ans = ans.min(dfs(&grid, &move_cost, &mut f, 0, i));
    }
    ans
}

fn dfs(
    grid: &Vec<Vec<i32>>,
    move_cost: &Vec<Vec<i32>>,
    f: &mut Vec<Vec<i32>>,
    x: usize,
    y: usize,
) -> i32 {
    if x == grid.len() - 1 {
        return grid[x][y];
    }
    if f[x][y] != -1 {
        return f[x][y];
    }

    let mut min = i32::MAX;
    for i in 0..grid[0].len() {
        let t = dfs(grid, move_cost, f, x + 1, i);
        min = min.min(t + move_cost[grid[x][y] as usize][i]);
    }

    f[x][y] = min + grid[x][y];
    f[x][y]
}

#[cfg(test)]
mod test {
    use crate::arr2d_to_vec2d;
    use crate::p2304::min_path_cost;

    #[test]
    fn test() {
        let ans = min_path_cost(
            arr2d_to_vec2d!([[5, 1, 2], [4, 0, 3]]),
            arr2d_to_vec2d!([
                [12, 10, 15],
                [20, 23, 8],
                [21, 7, 1],
                [8, 1, 13],
                [9, 10, 25],
                [5, 3, 2]
            ]),
        );
        assert_eq!(ans, 6);
    }
}
