pub fn detect_capital_use(word: String) -> bool {
    is_all_lowercase(&word) || is_all_uppercase(&word) || is_upper_camel_case(&word)
}

#[inline]
fn is_all_lowercase(s: &str) -> bool {
    s.chars().all(|v| v.is_ascii_lowercase())
}

#[inline]
fn is_all_uppercase(s: &str) -> bool {
    s.chars().all(|v| v.is_ascii_uppercase())
}

#[inline]
fn is_upper_camel_case(s: &str) -> bool {
    is_all_uppercase(&s[0..1]) && is_all_lowercase(&s[1..])
}

mod solution2 {
    pub fn detect_capital_use(word: String) -> bool {
        let cnt = word.chars().filter(|v| v.is_uppercase()).count();
        cnt == 0 || cnt == word.len() || cnt == 1 && word.chars().next().unwrap().is_uppercase()
    }
}
