pub fn min_cut(s: String) -> i32 {
    let s = s.as_bytes();
    let p = cal_palindrome_array(s);
    let mut memo = vec![-1; s.len()];
    dfs(s, 0, &p, &mut memo)
}

/// Time Complexity: O(n^3)
pub fn cal_palindrome_array(s: &[u8]) -> Vec<Vec<usize>> {
    let mut p = vec![];
    for i in 0..s.len() {
        p.push(vec![]);
        for j in i + 1..=s.len() {
            if (i..j).all(|k| s[k] == s[j - k + i - 1]) {
                p[i].push(j);
            }
        }
    }
    p
}

pub fn dfs(s: &[u8], pos: usize, p: &Vec<Vec<usize>>, memo: &mut Vec<i32>) -> i32 {
    if pos >= s.len() {
        return -1;
    }

    if memo[pos] != -1 {
        return memo[pos];
    }

    let mut min_cnt = s.len() as i32;

    for i in p[pos].iter().rev() {
        let cnt = dfs(s, *i, p, memo);
        min_cnt = min_cnt.min(cnt);
    }

    memo[pos] = min_cnt + 1;
    memo[pos]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cut() {
        assert_eq!(min_cut("aab".to_string()), 1);
        assert_eq!(min_cut("a".to_string()), 0);
        assert_eq!(min_cut("ab".to_string()), 1);
        assert_eq!(min_cut("racecar".to_string()), 0);
        assert_eq!(min_cut("aabb".to_string()), 1);
        assert_eq!(min_cut("abcba".to_string()), 0);
        assert_eq!(min_cut("abccba".to_string()), 0);
        assert_eq!(min_cut("abcdef".to_string()), 5);
    }
}
