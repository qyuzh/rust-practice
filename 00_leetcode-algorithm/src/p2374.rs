/// runs in O(n)/O(n)
pub fn edge_score(edges: Vec<i32>) -> i32 {
    if edges.is_empty() {
        return -1; // unreachable
    }
    let mut scores = vec![0; edges.len()];
    edges.into_iter().enumerate().for_each(|(from, to)| {
        scores[to as usize] += from;
    });
    let max = scores.iter().max().unwrap();
    scores
        .iter()
        .enumerate()
        .find_map(|(i, x)| if x == max { Some(i) } else { None })
        .unwrap() as i32
}
