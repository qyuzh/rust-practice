/// Find a and b.
/// 1. we have that XOR{nums} = a ^ b.
/// 2. divide nums into 2 sets that one has lowbit = 0, another has lowbit = 1.
pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let xor_all = nums.iter().fold(0, |acc, &v| acc ^ v);
    let lowbit = xor_all & -xor_all; // lowbit
    let mut ans = vec![0, 0];
    for &x in &nums {
        if (x & lowbit) == 0 {
            ans[0] ^= x;
        } else {
            ans[1] ^= x;
        }
    }
    ans
}
