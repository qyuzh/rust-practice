pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();

    let mut ans = 0;

    let mut f = vec![vec![-1; m]; n];
    for x in (0..n) {
        dfs(&grid, &mut f, x, 0);
        ans = ans.max(f[x][0]);
    }

    ans - 1 // moves = the number of cells - 1
}

fn dfs(grid: &Vec<Vec<i32>>, f: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    if f[x][y] != -1 {
        return f[x][y];
    }

    f[x][y] = 0;

    for (dx, dy) in [(-1, 1), (0, 1), (1, 1)] {
        if dx == -1 && x == 0 {
            continue;
        }

        let nx = (x as i32 + dx) as usize;
        let ny = y + dy as usize;

        if nx >= grid.len() || ny >= grid[0].len() {
            continue;
        }

        if grid[x][y] >= grid[nx][ny] {
            continue;
        }

        f[x][y] = f[x][y].max(dfs(grid, f, nx, ny));
    }

    f[x][y] += 1; // add current cell
    f[x][y]
}
