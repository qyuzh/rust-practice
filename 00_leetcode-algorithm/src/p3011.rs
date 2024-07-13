/// runs in O(n)/O(1)
pub fn can_sort_array(nums: Vec<i32>) -> bool {
    let mut max = 0;
    let mut i = 0;
    while i < nums.len() {
        let mut min_v = nums[i];
        let mut max_v = nums[i];
        let bits = nums[i].count_ones();
        i += 1;
        while i < nums.len() {
            if bits == nums[i].count_ones() {
                min_v = min_v.min(nums[i]);
                max_v = max_v.max(nums[i]);
            } else {
                break;
            }
            i += 1;
        }
        if max > min_v {
            return false;
        } else {
            max = max_v;
        }
    }
    true
}
