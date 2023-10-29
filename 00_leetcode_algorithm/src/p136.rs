/// P136, Single Number
/// we have that a ^ a ^ b = b
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, &v| acc ^ v)
}
