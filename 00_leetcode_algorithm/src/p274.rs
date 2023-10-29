/// v1: sort first then iter in O(nlogn)/O(1)
pub fn h_index(mut citations: Vec<i32>) -> i32 {
    citations.sort_unstable_by_key(|v| -*v);
    let mut h = 0;
    for (idx, &v) in citations.iter().enumerate() {
        if v > idx as i32 {
            h = h.max(idx as i32);
        }
    }
    h + 1
}
