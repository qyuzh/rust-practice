/// runs in O(nC)/O(1), in which n = s.len(), C = 26
pub fn get_smallest_string(s: String, k: i32) -> String {
    let s = s.as_bytes();
    let mut ans = vec![];
    let mut distance = 0;
    for &c in s {
        for cc in b'a'..=b'z' {
            let d = distance_char(cc, c) as i32;
            if d <= k - distance {
                ans.push(cc);
                distance += d;
                break;
            }
        }
    }
    String::from_utf8(ans).unwrap()
}

fn distance_char(a: u8, b: u8) -> u8 {
    if a >= b {
        (a - b).min(26 - a + b)
    } else {
        distance_char(b, a)
    }
}
