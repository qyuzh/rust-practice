/// has at least one trailing zero <=> the lowest bit is not 1
/// O(n)/O(1)
pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
    nums.iter()
        .fold(0, |r, v| r + if *v & 1 == 0 { 1 } else { 0 })
        >= 2
}

pub fn maximum_length(s: String) -> i32 {
    let s = s.as_bytes();
    let mut ans = -1;
    let mut l = 0;
    while l < s.len() {
        let mut r = l;
        while r < s.len() && s[r] == s[l] {
            // check
            let mut cnt = 0;
            let mut i = 0;
            let mut j = 0;
            while l + i < s.len() {
                if s[l + i] == s[l + j] {
                    i += 1;
                    j += 1;
                } else {
                    i = i - j + 1;
                    j = 0;
                }
                if l + j == r + 1 {
                    cnt += 1;

                    i = i - j + 1;
                    j = 0;
                }
            }
            if cnt >= 3 {
                ans = ans.max((r - l + 1) as i32);
            }

            r += 1;
        }
        l += 1
    }
    ans
}

/// O(n)/O(n)
pub fn maximum_length2(s: String) -> i32 {
    let s = s.as_bytes();

    let mut cnts = vec![std::collections::HashMap::new(); 26];
    let mut i = 0;
    while i < s.len() {
        let c = (s[i] - b'a') as usize;

        let mut j = 1;
        while i + j < s.len() && s[i + j] == s[i] {
            j += 1;
        }

        cnts[c].entry(j).and_modify(|v| *v += 1).or_insert(1);
        if j > 1 {
            cnts[c].entry(j - 1).and_modify(|v| *v += 2).or_insert(2);
        }
        if j > 2 {
            cnts[c].entry(j - 2).and_modify(|v| *v += 3).or_insert(3);
        }

        i += j;
    }

    let mut ans = -1;
    for ht in cnts.into_iter() {
        for (k, v) in ht.iter() {
            if *v >= 3 {
                ans = ans.max(*k as i32);
            }
        }
    }

    ans
}

///
pub fn can_make_palindrome_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::wc378::{maximum_length, maximum_length2};

    #[test]
    fn test_b() {
        let t = maximum_length("aaaa".into());
        assert_eq!(t, 2);
    }

    #[test]
    fn test_c() {
        let t = maximum_length2("abbbbbggggggyyyggggg".into());
        assert_eq!(t, 5);
    }
}
