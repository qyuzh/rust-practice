/// runs in O(n + nlogn)/O(1), in which n = coins.len()
pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
    coins.sort_unstable(); // O(nlogn)

    let mut ans = 0;

    let mut ob = 0;
    let mut i = 0;
    while i < coins.len() {
        if ob >= target {
            return ans;
        }
        if coins[i] <= ob + 1 {
            ob += coins[i];
            i += 1;
        } else {
            ob = ob + ob + 1;
            ans += 1;
        }
    }

    if ob >= target {
        return ans;
    }

    // Could be optimized to O(1)
    while ob < target {
        ob = ob + ob + 1;
        ans += 1;
    }

    ans
}
