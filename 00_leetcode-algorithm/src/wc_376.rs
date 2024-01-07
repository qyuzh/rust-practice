pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ht = vec![0; grid.len() * grid.len() + 1];
    for row in grid.iter() {
        for x in row.iter() {
            ht[*x as usize] += 1;
        }
    }
    let mut ans = vec![0, 0];
    for x in ht.iter().enumerate().skip(1) {
        if *x.1 == 2 {
            ans[0] = x.0 as i32
        } else if *x.1 == 0 {
            ans[1] = x.0 as i32
        }
    }
    ans
}

pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut ans = vec![];
    let mut i = 0;
    while i < nums.len() {
        let mut t = vec![0, 0, 0];
        t[0] = nums[i];
        t[1] = nums[i + 1];
        t[2] = nums[i + 2];
        if t[1] - t[0] > k || t[2] - t[1] > k || t[2] - t[0] > k {
            return vec![];
        }
        ans.push(t);
        i += 3;
    }
    ans
}

pub fn minimum_cost(mut nums: Vec<i32>) -> i64 {
    nums.sort_unstable();
    let mid = nums[nums.len() / 2] as i64;

    let mut x = mid;
    while !is_palindromic(x) {
        x += 1;
    }
    let r_sum: i64 = nums.iter().map(|&v| (v as i64 - x).abs()).sum();

    let mut y = mid;
    while !is_palindromic(y) {
        y -= 1;
    }
    let l_sum: i64 = nums.iter().map(|&v| (v as i64 - y).abs()).sum();

    l_sum.min(r_sum)
}

fn is_palindromic(y: i64) -> bool {
    let mut t = vec![];
    let mut x = y;
    while x > 0 {
        t.push(x % 10);
        x /= 10;
    }
    for i in 0..t.len() {
        if t[i] != t[t.len() - i - 1] {
            return false;
        }
    }
    true
}
