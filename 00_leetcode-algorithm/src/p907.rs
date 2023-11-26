/// Get answer in O(n)/O(n) using mono-stack
///
/// # Edge Case
/// xxxaxxxa \
/// 01234567
pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    const MOD: usize = 1e9 as usize + 7;

    let mut right = vec![arr.len(); arr.len()];
    let mut q = vec![];
    for i in (0..arr.len()).rev() {
        while !q.is_empty() && arr[*q.last().unwrap()] > arr[i] {
            q.pop();
        }
        if !q.is_empty() {
            right[i] = *q.last().unwrap();
        }
        q.push(i);
    }

    let mut ans = 0;

    q.clear();
    for i in 0..arr.len() {
        while !q.is_empty() && arr[*q.last().unwrap()] >= arr[i] {
            q.pop();
        }
        if q.is_empty() {
            ans = (ans + (right[i] - i) * (i + 1) * arr[i] as usize) % MOD;
        } else {
            ans = (ans + (right[i] - i) * (i - *q.last().unwrap()) * arr[i] as usize) % MOD;
        }
        q.push(i);
    }

    ans as i32
}
