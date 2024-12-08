/// run in O(n)/O(n), in which n = s.len()
/// s, must be ascii only charactar
pub fn is_substring_present(s: String) -> bool {
    let rev_s: String = s.chars().rev().collect();

    let n = s.len(); // length of bytes

    for i in 0..n - 1 {
        // SAFETY: s only consists of ascii
        if rev_s.contains(&s[i..i + 2]) {
            return true;
        }
    }

    false
}

/// runs in O(n)/O(1)
pub fn count_substrings(s: String, c: char) -> i64 {
    let mut cnt = 0;

    for ch in s.chars() {
        if ch == c {
            cnt += 1;
        }
    }

    (cnt + 1) * cnt / 2
}

/// runs in O(n + C^2)/O(C), in which n = word.len(), C = 26
pub fn minimum_deletions(word: String, k: i32) -> i32 {
    let mut cnts = [0; 26];

    for &c in word.as_bytes() {
        cnts[(c - b'a') as usize] += 1;
    }

    cnts.sort_unstable();

    let t: Vec<i32> = cnts
        .iter()
        .filter_map(|&v| if v > 0 { Some(v) } else { None })
        .collect();

    let mut ans = i32::MAX;

    let mut pre = 0;
    for &x in t.iter() {
        let max = x + k;

        let mut sum = pre;

        for &xx in t.iter() {
            if xx > max {
                sum += xx - max;
            }
        }

        ans = ans.min(sum);

        pre += x;
    }

    ans
}

/// runs in O(n)/O(n), in which n = nums.len()
pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
    let mut pos = vec![];

    // 0 <= c <= 3
    let mut c = 0; // the length of continues-1

    for i in 0..nums.len() {
        if nums[i] == 0 {
            continue;
        }

        pos.push(i); // record the position of 1

        c = c.max(1);

        if i > 0 && nums[i - 1] == 1 {
            if i > 1 && nums[i - 2] == 1 {
                c = 3; // 3
            } else {
                c = c.max(2); // 2
            }
        }
    }

    let needed_c = c.min(k);

    if max_changes >= k - needed_c {
        return 0.max(needed_c as i64 - 1) // class 1: idx - 1, idx, idx + 1
                + (k - needed_c) as i64 * 2; // class 2: ops1 + ops2
    }

    let n = pos.len();

    let mut sum: Vec<i64> = vec![0; n + 1];

    for i in 0..n {
        sum[i + 1] = sum[i] + pos[i] as i64;
    }

    let mut ans = i64::MAX;
    let size = (k - max_changes) as usize;
    for r in size..=n {
        let l = r - size;

        let i = l + size / 2;

        let idx = pos[i];

        let s1 = (idx * (i - l)) as i64 - (sum[i] - sum[l]);
        let s2 = sum[r] - sum[i] - (idx * (r - i)) as i64;

        ans = ans.min(s1 + s2);
    }

    ans + max_changes as i64 * 2
}

#[cfg(test)]
mod test {
    use super::minimum_deletions;

    #[test]
    fn test_minimum_deletions() {
        let t = minimum_deletions("dabdcbdcdcd".into(), 2);
        assert_eq!(t, 2);
    }
}
