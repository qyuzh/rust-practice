use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

type SafeHashMap<K, V> = LazyLock<Mutex<HashMap<K, V>>>;

pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
    let mut max_prime = 0;
    let mut n = nums[0].len();
    for (i, a) in nums.iter().enumerate() {
        if is_prime(a[i]) {
            max_prime = max_prime.max(a[i]);
        }
        if is_prime(a[n - i - 1]) {
            max_prime = max_prime.max(a[n - i - 1]);
        }
    }
    max_prime
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }

    static PRIMES: SafeHashMap<i32, bool> = LazyLock::new(|| Mutex::new(HashMap::new()));

    let mut primes = PRIMES.lock().unwrap();

    if let Some(&is_prime) = primes.get(&n) {
        return is_prime;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            primes.insert(n, false);
            return false;
        }
        i += 1;
    }

    primes.insert(n, true);

    true
}
