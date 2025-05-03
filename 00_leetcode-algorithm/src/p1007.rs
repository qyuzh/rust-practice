/// Runs in `O(n)` time and `O(1)` space.
pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    let mut top_ht = [0; 7];
    let mut bottom_ht = [0; 7];
    let mut same_ht = [0; 7];

    for i in 0..tops.len() {
        top_ht[tops[i] as usize] += 1;
        bottom_ht[bottoms[i] as usize] += 1;
        if tops[i] == bottoms[i] {
            same_ht[tops[i] as usize] += 1;
        }
    }

    for i in 1..=6 {
        let top = top_ht[i];
        let bottom = bottom_ht[i];
        let same = same_ht[i];
        if top + bottom - same == tops.len() {
            return (tops.len() - top.max(bottom)) as i32;
        }
    }

    -1
}
