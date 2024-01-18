pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    let mut cnts = [0; 26];
    s.as_bytes().iter().for_each(|&c| {
        cnts[(c - b'a') as usize] += 1;
    });

    let mut ans: Vec<u8> = vec![];
    loop {
        // 获取最后一个字符
        let last_ch = if let Some(&c) = ans.last() {
            (c - b'a') as usize
        } else {
            26 // guard
        };

        // 下一个字符: 不和last_ch相等
        let mut now_ch = 26; // 26 is guard
        let mut has_great = false;
        for (c, &cnt) in cnts.iter().enumerate().rev() {
            if cnt == 0 {
                continue;
            }
            if c != last_ch {
                now_ch = c;
                break;
            } else {
                has_great = true;
            }
        }

        if now_ch == 26 {
            break;
        }

        // 下一个字符插入的个数
        let push_num = if has_great {
            1
        } else {
            repeat_limit.min(cnts[now_ch])
        };

        for _ in 0..push_num {
            ans.push(b'a' + now_ch as u8);
        }
        cnts[now_ch] -= push_num;
    }

    // SAFETY: s is ascii
    unsafe { String::from_utf8_unchecked(ans) }
}
