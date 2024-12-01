/// The time complexity of the solve_n_queens function is O(N!), where N is the number of queens (or the size of the board).
/// This is because we are trying to place a queen in each row, and for each row, we have N choices (columns) to place the queen.
/// As we go deeper into the recursion, the number of choices decreases, leading to a factorial time complexity.
///
/// The space complexity is O(N^2) for the board and the result storage. The board is a 2D vector of size N x N,
/// and in the worst case, the result vector can store all possible solutions, each of which is a 2D vector of size N x N.
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut board = vec![vec!['.'; n as usize]; n as usize];
    let mut res = vec![];
    dfs(&mut board, 0, &mut res);
    res
}

fn dfs(board: &mut Vec<Vec<char>>, row: usize, res: &mut Vec<Vec<String>>) {
    // If we've placed queens in all rows, add the current board configuration to the results
    if row == board.len() {
        let mut tmp = vec![];
        for row in board.iter() {
            tmp.push(row.iter().collect());
        }
        res.push(tmp);
        return;
    }

    // Try placing a queen in each column of the current row
    for col in 0..board.len() {
        // Check if placing a queen at (row, col) is valid
        if is_valid(board, row, col) {
            // Place the queen
            board[row][col] = 'Q';
            // Recur to place queens in the next row
            dfs(board, row + 1, res);
            // Backtrack and remove the queen
            board[row][col] = '.';
        }
    }
}

fn is_valid(board: &[Vec<char>], row: usize, col: usize) -> bool {
    // Check the column for any other queens above the current row
    for r in &board[..row] {
        if r[col] == 'Q' {
            return false; // Another queen found in the same column
        }
    }

    // Check the upper left diagonal for any other queens
    let mut i = row as i32 - 1;
    let mut j = col as i32 - 1;
    while i >= 0 && j >= 0 {
        if board[i as usize][j as usize] == 'Q' {
            return false; // Another queen found in the upper left diagonal
        }
        i -= 1;
        j -= 1;
    }

    // Check the upper right diagonal for any other queens
    let mut i = row as i32 - 1;
    let mut j = col as i32 + 1;
    while i >= 0 && j < board.len() as i32 {
        if board[i as usize][j as usize] == 'Q' {
            return false; // Another queen found in the upper right diagonal
        }
        i -= 1;
        j += 1;
    }

    // No conflicts found, the position is valid
    true
}
