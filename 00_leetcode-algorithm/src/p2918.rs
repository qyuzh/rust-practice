/// Runs in O(n) time and O(1) space.
pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let sum1: i64 = nums1.iter().map(|&x| x as i64).sum();
    let sum2: i64 = nums2.iter().map(|&x| x as i64).sum();
    let zero1 = nums1.iter().filter(|&&x| x == 0).count();
    let zero2 = nums2.iter().filter(|&&x| x == 0).count();
    match (zero1, zero2) {
        (0, 0) => {
            if sum1 == sum2 {
                sum1
            } else {
                -1
            }
        }
        (0, _) => {
            if sum1 >= sum2 + zero2 as i64 {
                sum1
            } else {
                -1
            }
        }
        (_, 0) => {
            if sum2 >= sum1 + zero1 as i64 {
                sum2
            } else {
                -1
            }
        }
        _ => std::cmp::max(sum1 + zero1 as i64, sum2 + zero2 as i64),
    }
}
