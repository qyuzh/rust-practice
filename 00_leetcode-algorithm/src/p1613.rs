/// Greedy 最短路径算法的变体
/// get answer in O(nm * log(nm))/O(nm)
pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let (n, m) = (heights.len() as i32, heights[0].len() as i32);
    let mut vis = vec![vec![false; heights[0].len()]; heights.len()];
    let mut q = std::collections::BinaryHeap::new();
    q.push((0, heights.len() - 1, heights[0].len() - 1));
    while !q.is_empty() {
        let (d, x, y) = q.pop().unwrap();
        if x == 0 && y == 0 {
            return -d;
        }
        if vis[x][y] {
            continue;
        }
        vis[x][y] = true;
        for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if 0 <= nx && nx < n && 0 <= ny && ny < m && !vis[nx as usize][ny as usize] {
                let nd = (heights[nx as usize][ny as usize] - heights[x][y])
                    .abs()
                    .max(-d);
                q.push((-nd, nx as usize, ny as usize));
            }
        }
    }
    unreachable!()
}
