pub fn min_days(n: i32) -> i32 {
    let mut f = std::collections::HashMap::new();
    dfs(n, &mut f)
}

fn dfs(n: i32, f: &mut std::collections::HashMap<i32, i32>) -> i32 {
    if n <= 1 {
        return n;
    }

    if let Some(&v) = f.get(&n) {
        return v;
    }

    let ret = (dfs(n / 2, f) + n % 2).min(dfs(n / 3, f) + n % 3) + 1;

    f.insert(n, ret);

    ret
}
