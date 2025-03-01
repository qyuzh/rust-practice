/// a a b
/// aa b
/// palindrome_starting_with[i] = true  
pub fn partition(s: String) -> Vec<Vec<String>> {
    let s = s.as_bytes();

    let p = {
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
    };

    fn dfs(
        s: &[u8],
        pos: usize,
        p: &Vec<Vec<usize>>,
        res: &mut Vec<Vec<String>>,
        cur: &mut Vec<String>,
    ) {
        if pos >= s.len() {
            if !cur.is_empty() {
                res.push(cur.clone());
            }
            return;
        }
        for i in p[pos].iter() {
            cur.push(unsafe { String::from_utf8_unchecked(s[pos..*i].to_vec()) });
            dfs(s, *i, p, res, cur);
            cur.pop();
        }
    }

    let mut res = vec![];
    let mut cur = vec![];
    dfs(s, 0, &p, &mut res, &mut cur);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        let s = "aab".to_string();
        let result = partition(s);
        let expected = vec![
            vec!["a".to_string(), "a".to_string(), "b".to_string()],
            vec!["aa".to_string(), "b".to_string()],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_partition_single_char() {
        let s = "a".to_string();
        let result = partition(s);
        let expected = vec![vec!["a".to_string()]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_partition_no_palindrome() {
        let s = "abc".to_string();
        let result = partition(s);
        let expected = vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_partition_all_same_char() {
        let s = "aaa".to_string();
        let result = partition(s);
        let expected = vec![
            vec!["a".to_string(), "a".to_string(), "a".to_string()],
            vec!["a".to_string(), "aa".to_string()],
            vec!["aa".to_string(), "a".to_string()],
            vec!["aaa".to_string()],
        ];
        assert_eq!(result, expected);
    }
}
