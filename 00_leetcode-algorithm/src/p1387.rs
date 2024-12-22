/// Runs in O(nlogn) time.
pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let mut nums = vec![];
    for i in lo..=hi {
        nums.push((i, POWERS[i as usize]));
    }
    nums.sort_by(|a, b| a.1.cmp(&b.1));
    nums[k as usize - 1].0
}

const fn power_of(n: i32) -> i32 {
    let mut power = 0;
    let mut num = n;
    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = 3 * num + 1;
        }
        power += 1;
    }
    power
}

const fn init_powers<const N: usize>() -> [i32; N] {
    let mut powers = [0; N];
    let mut i = 1;
    while i < N {
        powers[i] = power_of(i as i32);
        i += 1;
    }
    powers
}

static POWERS: [i32; 1001] = init_powers();
