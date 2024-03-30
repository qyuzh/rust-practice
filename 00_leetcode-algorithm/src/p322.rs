/// `f[i] = min{f[i - k * coins[j]] + k}`
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut f = vec![usize::MAX >> 1; amount + 1];

    f[0] = 0;
    coins.iter().for_each(|&x| {
        if x as usize <= amount {
            f[x as usize] = 1
        }
    });

    for i in 1..=amount {
        if f[i] == 1 {
            continue;
        }
        for (j, &x) in coins.iter().enumerate() {
            let x = x as usize;
            for k in (0..=(i / x)) {
                f[i] = f[i].min(f[i - k * x] + k);
            }
        }
    }

    if f[amount] >= usize::MAX >> 1 {
        return -1;
    }

    f[amount] as i32
}

/// `f[i][j]`, 使用前j个硬币, 凑出金额i的最小硬币数
///
/// ```
/// f[i][j] = min{f[i][j-1], f[i-x][j-1] + 1, ..., f[i - kx][j-1] + k}, k = i / x, x = coins[j]
/// ```
///
/// let `i = i - x`,
/// ```
/// f[i-x][j] = min(f[i-x][j-1], f[i-2x][j-1]+1, ..., f[i-kx][j-1] + k - 1)
///           = min(f[i-x][j-1]+1, f[i-2x][j-1]+2, ..., f[i-kx][j-1] + k) - 1
/// ```
/// 其中, (`f[i-(k+1)x][j-1] + k`) 没有意义
///
/// `f[i][j] = min(f[i][j-1], f[i-x][j] + 1)`
///
pub fn coin_change2(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut f = vec![usize::MAX >> 1; amount + 1];

    f[0] = 0;
    coins.iter().for_each(|&x| {
        if x as usize <= amount {
            f[x as usize] = 1
        }
    });

    for i in 1..=amount {
        if f[i] == 1 {
            continue;
        }
        for (j, &x) in coins.iter().enumerate() {
            let x = x as usize;
            for k in (0..=(i / x)) {
                f[i] = f[i].min(f[i - k * x] + k);
            }
        }
    }

    if f[amount] >= usize::MAX >> 1 {
        return -1;
    }

    f[amount] as i32
}
