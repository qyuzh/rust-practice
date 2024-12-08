pub fn punishment_number(n: i32) -> i32 {
    let mut res = 1;
    for i in 2..n + 1 {
        if can_partition(&(i * i).to_string(), i) {
            res += i * i;
        }
    }
    res
}

/// num = 26
/// num*num = 1296 tat 1 + 29 + 6
fn can_partition(str: &str, num: i32) -> bool {
    if str.is_empty() {
        return num == 0;
    }
    if num < 0 {
        return false;
    }

    let mut t = 0;
    for (idx, &c) in str.as_bytes().iter().enumerate() {
        t = t * 10 + (c - b'0') as i32;
        if can_partition(&str[idx + 1..], num - t) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::p2698::{can_partition, punishment_number};

    #[test]
    fn test_can_partition() {
        assert!(can_partition("", 0));
        assert!(can_partition("1", 1));
        assert!(can_partition("1296", 36));
    }

    #[test]
    fn test_punishment_number() {
        assert_eq!(punishment_number(1), 1);
        assert_eq!(punishment_number(10), 182);
        assert_eq!(punishment_number(37), 1478);
    }
}
