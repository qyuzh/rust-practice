/// 1. swap two rows
/// 2. swap two columns
pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
    // 第一行，0 和 1 的个数之差不能超过 1
    let first_row = &board[0];
    let mut row_cnt = [0i32; 2];
    for &x in first_row.iter() {
        row_cnt[x as usize] += 1;
    }
    if (row_cnt[0] - row_cnt[1]).abs() > 1 {
        return -1;
    }

    // 第一列，0 和 1 的个数之差不能超过 1
    let first_col = board.iter().map(|row| row[0]).collect::<Vec<_>>();
    let mut col_cnt = [0i32; 2];
    for &x in first_col.iter() {
        col_cnt[x as usize] += 1;
    }
    if (col_cnt[0] - col_cnt[1]).abs() > 1 {
        return -1;
    }

    // 每一行和第一行比较，要么完全相同，要么完全不同
    for row in &board {
        let same = row[0] == first_row[0];
        for (x, y) in row.iter().zip(first_row) {
            if (x == y) != same {
                return -1;
            }
        }
    }

    min_swap(first_row, row_cnt) + min_swap(&first_col, col_cnt)
}

fn min_swap(arr: &[i32], cnt: [i32; 2]) -> i32 {
    let x0 = (cnt[1] > cnt[0]) as i32;
    let diff = arr
        .iter()
        .enumerate()
        .map(|(i, &x)| (i % 2) as i32 ^ x ^ x0)
        .sum::<i32>();
    let n = arr.len() as i32;
    if n % 2 > 0 {
        diff / 2
    } else {
        diff.min(n - diff) / 2
    }
}
