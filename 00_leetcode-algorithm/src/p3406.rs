pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
    if nums.len() % 2 != 0 {
        return false;
    }
    let mut hm = [0; 101];
    for x in nums {
        hm[x as usize] += 1;
        if hm[x as usize] > 2 {
            return false;
        }
    }
    true
}
