/// find the width of columns of a grid
pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
    assert!(grid.len() >= 1, "grid's len can not be 0");

    let mut ret = vec![0; grid[0].len()];

    for j in (0..grid[0].len()) {
        for i in (0..grid.len()) {
            ret[j] = ret[j].max(find_len(grid[i][j]));
        }
    }

    ret
}

#[inline]
fn find_len(mut x: i32) -> i32 {
    let mut cnt = 0;

    if x <= 0 {
        cnt += 1;
        x = -x;
    }

    while x > 0 {
        x /= 10;
        cnt += 1;
    }

    cnt
}
