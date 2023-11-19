pub fn find_the_longest_balanced_substring(s: String) -> i32 {
    let (mut a, mut b) = (0, 1);
    let mut ans = 0;
    let bytes = s.as_bytes();
    for i in 1..bytes.len() {
        if bytes[i] == bytes[i - 1] {
            b += 1;
        } else {
            if bytes[i] == b'0' {
                ans = ans.max(a.min(b) * 2);
            }
            a = b;
            b = 1;
        }
    }
    if bytes[bytes.len() - 1] == b'1' {
        ans = ans.max(a.min(b) * 2);
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::p2609::find_the_longest_balanced_substring;

    #[test]
    fn test() {
        assert_eq!(
            find_the_longest_balanced_substring("01000111".to_string()),
            6,
            "case 1"
        );
        assert_eq!(
            find_the_longest_balanced_substring("010".to_string()),
            2,
            "case 2"
        );
        assert_eq!(
            find_the_longest_balanced_substring("10".to_string()),
            0,
            "case 3"
        );
    }
}
