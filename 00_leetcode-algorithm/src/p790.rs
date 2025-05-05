/// Runs in O(n) time and O(1) space.
pub fn num_tilings(n: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let mut f = [0; 1001];
    f[0] = 1;
    f[1] = 1;
    f[2] = 2;
    for i in 3..=n as usize {
        f[i] = (f[i - 1] * 2 % MOD + f[i - 3]) % MOD;
    }
    f[n as usize]
}

mod tests {
    use super::*;

    #[test]
    fn test_num_tilings() {
        assert_eq!(num_tilings(1), 1);
        assert_eq!(num_tilings(2), 2);
        assert_eq!(num_tilings(3), 5);
        assert_eq!(num_tilings(4), 11);
        assert_eq!(num_tilings(5), 24);
        assert_eq!(num_tilings(6), 53);
        assert_eq!(num_tilings(7), 117);
        assert_eq!(num_tilings(8), 258);
    }
}
