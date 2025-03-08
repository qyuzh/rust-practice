pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    let vowels = [b'a', b'e', b'i', b'o', b'u'];
    let mut ans = 0;
    for i in left..=right {
        let w = &words[i as usize];
        if vowels.contains(&w.as_bytes()[0]) && vowels.contains(&w.as_bytes()[w.len() - 1]) {
            ans += 1;
        }
    }
    ans
}
