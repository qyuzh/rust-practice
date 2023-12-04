pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for i in 1..mountain.len() - 1 {
        if mountain[i - 1] < mountain[i] && mountain[i] > mountain[i + 1] {
            ans.push(i as i32);
        }
    }
    ans
}

/// get answer in O(nlogn + target)/O(1)
///
/// [analysis](http://blog.qyuzh.com/algorithm/leetcode/wc-374#b-minimum-number-of-coins-to-be-added)
pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
    coins.sort_unstable();
    let mut ans = 0;
    // [1, s - 1] is obtainable, s is not
    let (mut s, mut i) = (1, 0);
    while s <= target {
        if i < coins.len() && coins[i] <= s {
            // s is obtainable
            s += coins[i];
            i += 1;
        } else {
            // [s, coins[i]] is not obtainable
            s += s;
            ans += 1;
        }
    }
    ans
}

/// get ans in O(C^2 * n)/O(C)
/// keywords: sliding window
pub fn count_complete_substrings(word: String, k: i32) -> i32 {
    let mut ans = 0;
    for i in 1..=26 {
        ans += helper_c(word.as_bytes(), k, i);
    }
    ans
}

/// get ans in O(C * n)/O(C)
fn helper_c(word: &[u8], k: i32, cnt: i32) -> i32 {
    if k * cnt > word.len() as i32 {
        return 0;
    }
    let mut ans = 0;
    let size = (k * cnt) as usize; // the size of sliding window is k * cnt

    let mut ht = [0; 26];
    let (mut l, mut r) = (0usize, 0usize);
    while r < word.len() {
        let i = (word[r] - b'a') as usize;
        if size > 1 && r > 0 {
            let j = (word[r - 1] - b'a') as usize;
            if i > j + 2 || i + 2 < j {
                l = r;
                ht.fill(0);
            }
        }
        ht[i] += 1;
        if r - l + 1 == size {
            let mut flag = true;
            for i in 0..26 {
                if ht[i] > 0 && ht[i] != k {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans += 1;
            }
            ht[(word[l] - b'a') as usize] -= 1;
            l += 1;
        }
        r += 1;
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::wc374::count_complete_substrings;

    #[test]
    fn test_c() {
        let t = count_complete_substrings("gvgvvgv".to_string(), 2);
        assert_eq!(t, 1);
        let t = count_complete_substrings("jjjqq".to_string(), 1);
        assert_eq!(t, 5);
    }
}
