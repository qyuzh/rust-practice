const DIR: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

// runs in O(n + m)/O(1)
pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
    let n = board.len() as isize;
    let m = board[0].len() as isize;
    for (dx, dy) in DIR {
        let mut has_reversed_color = false;
        let (mut x, mut y) = (r_move as isize + dx, c_move as isize + dy);
        while 0 <= x && x < n && 0 <= y && y < m {
            if let c @ ('B' | 'W') = board[x as usize][y as usize] {
                if c != color {
                    has_reversed_color = true;
                } else if has_reversed_color {
                    return true;
                } else {
                    break;
                }
                x += dx;
                y += dy;
            } else {
                break;
            }
        }
    }
    false
}
