/// priority queue in O(klogn)
pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    use std::collections::BinaryHeap;
    let mut pq: BinaryHeap<i64> = gifts.into_iter().map(|v| v as i64).collect();
    for _ in 0..k {
        let val = pq.pop().unwrap();
        pq.push((val as f64).sqrt() as i64);
    }
    pq.into_iter().sum()
}