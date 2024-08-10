/// P2940 - Finding building where alice and bob can meet
///
/// runs in `O(nm)` where `n = height.len()`, `m = queries.len()`
pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![-1; queries.len()];
    queries.iter().enumerate().for_each(|(idx, q)| {
        ans[idx] = find_leftmost_meet_building(&heights, (q[0] as usize, q[1] as usize));
    });
    ans
}

/// runs in `O(n)` where `n = heights.len()`
fn find_leftmost_meet_building(heights: &[i32], (a, b): (usize, usize)) -> i32 {
    // 1 both don't move
    if a == b {
        return a as i32;
    }

    // 2 one move to another
    let mut max_p = a.max(b);
    let mut min_p = a.min(b);
    if heights[min_p] < heights[max_p] {
        return max_p as i32;
    }

    // 3 both move
    let mut mp = max_p;
    while mp < heights.len() {
        if heights[a] < heights[mp] && heights[b] < heights[mp] {
            return mp as i32;
        }
        mp += 1;
    }

    // not possible
    -1
}
