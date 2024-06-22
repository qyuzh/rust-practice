/// runs in O(n)/O(n)
pub fn smallest_beautiful_string(s: String, k: i32) -> String {
    if s.is_empty() {
        return "".into();
    }

    let k = k as u32 + 'a' as u32;

    let mut s: Vec<_> = s.chars().map(|v| v as u32).collect();

    let mut n = s.len();
    let mut i = n - 1;
    s[i] += 1;

    #[allow(clippy::never_loop)]
    while i < n {
        #[cfg(test)]
        let t = String::from_iter(s.clone().into_iter().map(|v| char::from_u32(v).unwrap()));
        if s[i] == k {
            // 需要进位

            if i == 0 {
                // 无法进位
                return "".into();
            }

            // 进位
            s[i] = 'a' as u32;
            i -= 1;
            s[i] += 1;
        } else if (i > 0 && s[i] == s[i - 1]) || (i > 1 && s[i] == s[i - 2]) {
            // 如果s[i]和左侧的子符形成回文串, 则继续增加s[i]
            s[i] += 1;
        } else {
            // 检查后面是否有回文串
            i += 1;
        }
    }

    String::from_iter(s.into_iter().map(|v| char::from_u32(v).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_beautiful_string() {
        let t = smallest_beautiful_string("abcz".into(), 26);
        assert_eq!(t, "abda");
    }
}
