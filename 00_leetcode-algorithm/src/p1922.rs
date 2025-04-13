const MOD: i64 = 1_000_000_007;

pub fn count_good_numbers(n: i64) -> i32 {
    let half_n = n / 2;
    let pow_4 = fast_mod_exp(4, half_n); // 2, 3, 5, 7
    let pow_5 = fast_mod_exp(5, n - half_n); // 0, 2, 4, 6, 8
    (pow_4 * pow_5 % MOD) as i32
}

/// Computes (base^exp) % MOD using fast exponentiation.
/// Time complexity: O(log(exp))
fn fast_mod_exp(base: i64, exp: i64) -> i64 {
    let mut result = 1;
    let mut base = base % MOD;
    let mut exp = exp;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        exp >>= 1;
    }
    result
}
