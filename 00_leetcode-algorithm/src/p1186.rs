/// runs in O(n)/O(1) by dp
pub fn maximum_sum(arr: Vec<i32>) -> i32 {
    let mut ans = i32::MIN >> 1;
    let mut f0 = ans; // f0_i, the max value of subarray that ends with i and don't delete number
    let mut f1 = ans; // f1_i, the max value of subarray that ends with i and delete a number
    for x in arr {
        f1 = f0.max(f1 + x);
        f0 = f0.max(0) + x;
        ans = ans.max(f0.max(f1));
    }
    ans
}
