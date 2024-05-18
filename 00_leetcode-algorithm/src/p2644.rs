/// runs in O(nm)/O(1)
pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
    let mut max_div_score = 0;
    let mut num = i32::MAX;
    divisors.iter().for_each(|&v| {
        let mut cnt = 0;
        nums.iter().for_each(|&x| {
            if x % v == 0 {
                cnt += 1
            }
        });
        match max_div_score.cmp(&cnt) {
            std::cmp::Ordering::Less => {
                num = v;
                max_div_score = cnt
            }
            std::cmp::Ordering::Equal => num = num.min(v),
            std::cmp::Ordering::Greater => {}
        }
    });
    num
}
