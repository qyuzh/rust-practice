pub fn count_good_integers(n: i32, k: i32) -> i64 {
    let f = factorial(n);
    let mut ans = 0;
    let mut ht = std::collections::HashSet::new();
    let base = 10i32.pow(((n - 1) / 2) as u32);
    for x in base..(base * 10) {
        let mut s = x.to_string();
        let suffix = &s.chars().rev().skip((n % 2) as usize).collect::<String>();
        s += suffix;

        if s.parse::<i64>().unwrap() % k as i64 != 0 {
            continue;
        }

        unsafe {
            s.as_bytes_mut().sort();
        }

        if (ht.contains(&s)) {
            continue;
        }

        ans += find_cnt(s.as_bytes(), n, &f) as i64;

        ht.insert(s);
    }

    ans
}

fn factorial(n: i32) -> Vec<i32> {
    let mut fact = vec![1; (n + 1) as usize];
    for i in 1..=n {
        fact[i as usize] = fact[(i - 1) as usize] * i;
    }
    fact
}

fn find_cnt(s: &[u8], n: i32, f: &[i32]) -> i32 {
    let mut cnt = [0; 10];

    for &c in s {
        cnt[(c - b'0') as usize] += 1;
    }

    let mut ans = (n - cnt[0]) * f[(n - 1) as usize];
    for &v in &cnt {
        ans /= f[v as usize];
    }

    ans
}
