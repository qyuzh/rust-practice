/// get ans in O(n)/O(m), in which n = trips.len(), m = max{to_i}
///
/// keywords: difference array
pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut locs = vec![0; 1001];
    for t in trips.iter() {
        locs[t[1] as usize] += t[0];
        locs[t[2] as usize] -= t[0];
    }
    let mut pre_sum = 0;
    for &x in locs.iter() {
        pre_sum += x;
        if pre_sum > capacity {
            return false;
        }
    }
    true
}
