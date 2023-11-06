/// DFA
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut a = 0;
    let mut b = 0;
    for &v in nums.iter() {
        b = b ^ v & !a;
        a = a ^ v & !b;
    }
    b
}
