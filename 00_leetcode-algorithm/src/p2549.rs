//! n = 1 ans = 1
//! n = 2 ans = 2
//! n = 3 ans = 2, 3

pub fn distinct_integers(n: i32) -> i32 {
    if n == 1 {
        1
    } else {
        n - 1
    }
}
