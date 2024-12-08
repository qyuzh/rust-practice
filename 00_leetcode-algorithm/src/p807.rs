/// runs in O(n*m)/O(n+m)
pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut max_rows = vec![0; m];
    let mut max_cols = vec![0; n];
    for (i, r) in grid.iter().enumerate() {
        max_rows[i] = *r.iter().max().unwrap();
    }
    for (i, max_col) in max_cols.iter_mut().enumerate() {
        for row in &grid {
            *max_col = (*max_col).max(row[i]);
        }
    }
    let mut ans = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &x) in row.iter().enumerate() {
            ans += max_rows[i].min(max_cols[j]) - x;
        }
    }
    ans
}
