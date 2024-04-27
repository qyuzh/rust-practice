pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    const SHIFT: i32 = 9;
    let mut ans = vec![];
    for mask in (0..(1i32 << SHIFT)) {
        if mask.count_ones() != k as u32 {
            continue;
        }
        let mut sum = 0;
        let mut res = vec![];
        for i in (0..SHIFT) {
            if mask >> i & 1 == 1 {
                sum += (i + 1);
                res.push((i + 1));
            }
        }
        if sum == n {
            ans.push(res)
        }
    }
    ans
}
